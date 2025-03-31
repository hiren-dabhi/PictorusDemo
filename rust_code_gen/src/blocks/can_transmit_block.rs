use alloc::vec::Vec;

use embedded_can::Frame;

use protocols::CanProtocol;
use utils::BlockData;

// Ideally this would not have to return a new vec, but that added a lot of complexity
// in the generated code. Possible we can simplify once we're generating our own CAN structs
type TxCallback<T> = fn(&[&BlockData], &mut T) -> Result<Vec<u8>, ()>;
pub struct CanTransmitBlock<T> {
    name: &'static str,
    frame_id: embedded_can::Id,
    tx_cb: TxCallback<T>,
    msg: T,
}

impl<T> CanTransmitBlock<T> {
    pub fn new(
        name: &'static str,
        frame_id: embedded_can::Id,
        tx_cb: TxCallback<T>,
        msg: T,
    ) -> Self {
        CanTransmitBlock {
            name,
            frame_id,
            tx_cb,
            msg,
        }
    }

    pub fn run<P: CanProtocol>(&mut self, inputs: &[&BlockData], proto: &mut P) {
        log::debug!("{}: Running", self.name);
        let data = if let Ok(data) = (self.tx_cb)(inputs, &mut self.msg) {
            data
        } else {
            log::warn!("{}: Failed to encode message data", self.name);
            return;
        };

        let frame = if let Some(frame) = P::Frame::new(self.frame_id, &data) {
            frame
        } else {
            log::warn!("{}: Failed to create frame", self.name);
            return;
        };

        let res = proto.transmit(&frame);
        if let Err(e) = res {
            log::warn!("{}: Failed to transmit frame: {:?}", self.name, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use embedded_can::StandardId;

    use protocols::MockCanProtocol;

    struct TestMessage {
        pub a: f32,
        pub b: f32,
    }

    impl TestMessage {
        fn new() -> Self {
            TestMessage { a: 0., b: 0. }
        }
    }

    fn test_tx_cb(inputs: &[&BlockData], msg: &mut TestMessage) -> Result<Vec<u8>, ()> {
        msg.a = inputs[0].scalar() as f32;
        msg.b = inputs[1].scalar() as f32;
        Ok(vec![msg.a as u8, msg.b as u8])
    }

    #[test]
    fn test_can_transmit_block() {
        let mut proto = MockCanProtocol::new();
        let frame_id = embedded_can::Id::Standard(StandardId::new(123).unwrap());
        let mut block = CanTransmitBlock::new("test", frame_id, test_tx_cb, TestMessage::new());

        proto.expect_transmit().times(1).returning(move |frame| {
            assert_eq!(frame.id(), frame_id,);
            assert_eq!(frame.data(), &[42, 99]);
            Ok(None)
        });
        block.run(
            &[&BlockData::from_scalar(42.), &BlockData::from_scalar(99.)],
            &mut proto,
        );
    }
}
