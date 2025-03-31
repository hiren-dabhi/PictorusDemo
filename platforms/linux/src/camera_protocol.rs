use log::{debug, warn};
use protocols::CameraProtocol;
use std::path::Path;
use std::process::Command;
use utils::PictorusError;

pub struct CameraConnection {}
impl CameraConnection {
    pub fn new() -> Result<Self, PictorusError> {
        debug!("Creating Camera");
        Ok(CameraConnection {})
    }
}

impl CameraProtocol for CameraConnection {
    fn capture(&self, photo_dir: &str, jpeg_quality: u8) {
        let filename = format!("photo_{}.jpg", chrono::Local::now().to_rfc3339());
        let photo_path = Path::new(photo_dir).join(filename);
        let photo_path = photo_path.to_str().unwrap();
        debug!("Capturing photo to location: {}", photo_path);
        let res = Command::new("libcamera-jpeg")
            .args([
                "-n",
                "-t1",
                "-o",
                photo_path,
                "-q",
                &jpeg_quality.to_string(),
            ])
            .output();

        if let Err(err) = res {
            warn!("failed to execute command with output: {}", err);
        };
    }
}
