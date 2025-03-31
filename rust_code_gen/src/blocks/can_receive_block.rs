use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;

use embedded_can::Frame;
use log::{debug, warn};
use protocols::CanProtocol;

use crate::{stale_tracker::StaleTracker, traits::IsValid};
use utils::BlockData;

type RxCallback<T> = fn(&T, &mut [BlockData]);
pub struct CanReceiveBlock<T> {
    name: &'static str,
    frame_id: embedded_can::Id,
    payload_len: usize,
    rx_cb: RxCallback<T>,
    pub stale_check: StaleTracker,
    pub data: Vec<BlockData>,
}

impl<'a, T: core::convert::TryFrom<&'a [u8]> + Debug> CanReceiveBlock<T>
where
    <T as TryFrom<&'a [u8]>>::Error: Debug,
{
    pub fn new(
        name: &'static str,
        frame_id: embedded_can::Id,
        signal_count: usize,
        payload_len: usize,
        rx_cb: RxCallback<T>,
        stale_age_ms: f64,
    ) -> Self {
        CanReceiveBlock {
            name,
            data: vec![BlockData::from_scalar(0.0); signal_count],
            frame_id,
            rx_cb,
            payload_len,
            stale_check: StaleTracker::from_ms(stale_age_ms),
        }
    }

    pub fn run(&mut self, proto: &'a mut impl CanProtocol, app_time_s: f64) {
        debug!("{}: Running", self.name);
        let frame = proto
            .read_frames()
            .iter()
            .rfind(|frame| frame.id() == self.frame_id);

        let Some(frame) = frame else {
            debug!("{}: No Frames to process", self.name);
            return;
        };
        let data = frame.data();
        let msg = T::try_from(&data[..self.payload_len]);

        match msg {
            Ok(msg) => {
                (self.rx_cb)(&msg, &mut self.data);
                self.stale_check.mark_updated(app_time_s);
                debug!("{}: Received message: {:#?}", self.name, msg)
            }
            Err(e) => {
                warn!("{}: Failed to parse message: {:?}", self.name, e);
            }
        }
    }
}

impl<T> IsValid for CanReceiveBlock<T> {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryFrom;
    use embedded_can::StandardId;
    use protocols::{MockCanProtocol, MockFrame};

    #[derive(Debug)]
    struct TestMessage {
        a: f32,
        b: f32,
    }

    impl<'a> TryFrom<&'a [u8]> for TestMessage {
        type Error = &'static str;

        fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
            if data.len() != 8 {
                return Err("Invalid data length");
            }

            let a = f32::from_le_bytes(data[0..4].try_into().unwrap());
            let b = f32::from_le_bytes(data[4..8].try_into().unwrap());

            Ok(TestMessage { a, b })
        }
    }

    fn test_rx_cb(msg: &TestMessage, data: &mut [BlockData]) {
        data[0] = BlockData::from_scalar(msg.a as f64);
        data[1] = BlockData::from_scalar(msg.b as f64);
    }

    #[test]
    fn test_can_receive_block() {
        let mut proto = MockCanProtocol::new();
        let frame_id = embedded_can::Id::Standard(StandardId::new(123).unwrap());
        let mut block =
            CanReceiveBlock::<TestMessage>::new("test", frame_id, 2, 8, test_rx_cb, 1000.);

        let a_val: f32 = 1.0;
        let b_val: f32 = 2.0;

        let data = [a_val.to_le_bytes().to_vec(), b_val.to_le_bytes().to_vec()].concat();
        let frame = MockFrame::new(frame_id, &data).unwrap();
        proto.expect_read_frames().return_const(vec![frame]);

        block.run(&mut proto, 0.0);

        assert_eq!(block.data[0].scalar(), a_val as f64);
        assert_eq!(block.data[1].scalar(), b_val as f64);
        assert!(block.is_valid(0.01).any());
    }
}
