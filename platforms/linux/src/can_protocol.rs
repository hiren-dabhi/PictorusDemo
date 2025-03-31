use embedded_can::{nb::Can, Frame as EmbeddedFrame};
use protocols::CanProtocol;
use socketcan::{CanFrame, CanSocket, Socket};
use utils::PictorusError;

const ERR_TYPE: &str = "CanProtocol";

pub struct CanConnection {
    socket: CanSocket,
    frames: Vec<CanFrame>,
    stale: bool,
}

impl CanConnection {
    pub fn new(iface: &str) -> Result<Self, PictorusError> {
        let socket = CanSocket::open(iface).map_err(|err| {
            PictorusError::new(
                ERR_TYPE.into(),
                format!(
                    "Failed to open CAN socket on interface: {} ({})",
                    iface, err
                ),
            )
        })?;

        socket.set_nonblocking(true).map_err(|err| {
            PictorusError::new(
                ERR_TYPE.into(),
                format!(
                    "Failed to set CAN socket to non-blocking mode: {} ({})",
                    iface, err
                ),
            )
        })?;

        Ok(Self {
            socket,
            frames: vec![],
            stale: true,
        })
    }
}

impl Can for CanConnection {
    type Frame = CanFrame;
    type Error = socketcan::Error;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        self.socket.transmit(frame)
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        self.socket.receive()
    }
}

impl CanProtocol for CanConnection {
    fn read_frames(&mut self) -> &[impl EmbeddedFrame] {
        if !self.stale {
            return &self.frames;
        }

        while let Ok(frame) = self.receive() {
            self.frames.push(frame);
        }

        self.stale = false;
        &self.frames
    }

    fn flush(&mut self) {
        self.stale = true;
        self.frames.clear();
    }
}
