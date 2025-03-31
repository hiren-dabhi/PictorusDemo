use crate::block_data::BlockData;

pub trait IsValid {
    fn is_valid(&self, app_time_s: f64) -> BlockData;
}
