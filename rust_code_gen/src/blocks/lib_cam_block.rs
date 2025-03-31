use crate::{
    block_data::{BlockData, BlockDataType},
    thread_manager::ThreadManager,
};
use protocols::CameraProtocol;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// Control a Camera
pub struct LibCameraBlock {
    name: &'static str,
    should_capture: Arc<AtomicBool>,
}

// TODO: I don't know if this will acutally work the way I changed it
impl LibCameraBlock {
    pub fn new(
        name: &'static str,
        photo_dir: &str,
        jpeg_quality: f64,
        thread_manager: &mut ThreadManager,
        proto: impl CameraProtocol + 'static,
    ) -> LibCameraBlock {
        // TODO: This is really dumb. In practice we're creating a new protocol for each block
        // Would be better to have the protocol manage any necessary threads,
        // and just pass the protocol in by ref for the "run" function.
        // This is how we deal with other I/O blocks and it simplifies things a bit
        // TODO: This just unwraps the protocol now so we won't get a helpful error on failure. Would also be resolved by fixing above ^^^
        let should_capture = Arc::new(AtomicBool::new(false));

        let capture = should_capture.clone();
        let photo_dir = String::from(photo_dir);
        let jpeg_quality = jpeg_quality as u8;
        thread_manager.register(move || {
            if capture.load(Ordering::Relaxed) {
                proto.capture(&photo_dir, jpeg_quality);
            }
        });

        LibCameraBlock {
            name,
            should_capture,
        }
    }
    pub fn run(&mut self, input: &BlockData) {
        match input.get_type() {
            BlockDataType::Scalar => {
                self.should_capture.store(input.any(), Ordering::Relaxed);
            }
            _ => panic!(
                "Can only command Camera {} with scalar/boolean input",
                self.name
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use protocols::MockCameraProtocol;

    #[test]
    fn test_camera_block_takes_photo() {
        let mut thread_manager = ThreadManager::new(0.1);
        let photo_dir = "/foo/bar";
        let mut proto = MockCameraProtocol::new();
        proto
            .expect_capture()
            .with(eq(photo_dir), eq(50))
            .return_const(());

        let mut block = LibCameraBlock::new("Camera1", photo_dir, 50., &mut thread_manager, proto);
        block.run(&BlockData::from_scalar(1.0));
    }

    #[test]
    fn test_camera_block_does_not_take_photo() {
        let mut thread_manager = ThreadManager::new(0.1);
        let photo_dir = "/foo/bar";
        let proto = MockCameraProtocol::new();

        let mut block = LibCameraBlock::new("Camera1", photo_dir, 100., &mut thread_manager, proto);
        block.run(&BlockData::from_scalar(0.0));
    }
}
