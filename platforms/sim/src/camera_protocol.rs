use log::debug;
use protocols::CameraProtocol;

pub struct CameraConnection;

impl CameraProtocol for CameraConnection {
    fn capture(&self, _: &str, _: u8) {
        debug!("Capturing photo");
    }
}

pub fn create_camera_connection() -> CameraConnection {
    CameraConnection {}
}
