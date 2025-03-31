use crate::block_data::BlockData;
use chrono::{DateTime, Datelike, Duration, Local, Timelike};
use log::debug;

#[derive(strum::EnumString)]
pub enum SystemTimeEnum {
    Epoch,
    Second,
    Minute,
    Hour,
    DayLunar,
    DayOrdinal,
    Month,
    Year,
}

pub struct SystemTimeBlock {
    pub name: &'static str,
    pub data: BlockData,
    pub method: SystemTimeEnum,
    pub local_time_at_start: DateTime<Local>,
}

impl SystemTimeBlock {
    pub fn new(
        name: &'static str,
        method: &str,
        app_init_time: DateTime<Local>,
    ) -> SystemTimeBlock {
        SystemTimeBlock {
            name,
            method: method.parse().unwrap(),
            data: BlockData::from_scalar(0.0),
            local_time_at_start: app_init_time,
        }
    }
    pub fn run(&mut self, app_time_s: f64) {
        // Since simulations can run faster than realtime, we'll use the delta between syatem start
        // and now, as measured by app_time, for system clock.
        let time_now = self.local_time_at_start
            + Duration::try_milliseconds((app_time_s * 1000.) as i64).unwrap();
        match self.method {
            SystemTimeEnum::Epoch => {
                self.data.set_scalar(time_now.timestamp() as f64);
            }
            SystemTimeEnum::Second => {
                self.data.set_scalar(time_now.second() as f64);
            }
            SystemTimeEnum::Minute => {
                self.data.set_scalar(time_now.minute() as f64);
            }
            SystemTimeEnum::Hour => {
                self.data.set_scalar(time_now.hour() as f64);
            }
            SystemTimeEnum::DayLunar => {
                self.data.set_scalar(time_now.day() as f64);
            }
            SystemTimeEnum::DayOrdinal => {
                self.data.set_scalar(time_now.ordinal() as f64);
            }
            SystemTimeEnum::Month => {
                self.data.set_scalar(time_now.month() as f64);
            }
            SystemTimeEnum::Year => {
                self.data.set_scalar(time_now.year() as f64);
            }
        }
        debug!("{} data: {:?}", self.name, self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_time_block_epoch() {
        let system_time_at_app_start_s = chrono::offset::Local::now();
        let mut block = SystemTimeBlock::new("SystemTime1", "Epoch", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(
            block.data.scalar(),
            system_time_at_app_start_s.timestamp() as f64
        );

        block.run(1.0);
        assert_eq!(
            block.data.scalar(),
            system_time_at_app_start_s.timestamp() as f64 + 1.0
        );
    }

    #[test]
    fn test_system_time_block_second() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 7, 27, 1, 2, 3)
            .unwrap();
        let mut block = SystemTimeBlock::new("SystemTime1", "Second", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 3.0);

        block.run(2.0);
        assert_eq!(block.data.scalar(), 5.0);

        // Increments in integer seconds
        block.run(2.99);
        assert_eq!(block.data.scalar(), 5.0);
        block.run(3.00);
        assert_eq!(block.data.scalar(), 6.0);
    }

    #[test]
    fn test_system_time_block_minute() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 7, 27, 1, 2, 3)
            .unwrap();
        let mut block = SystemTimeBlock::new("SystemTime1", "Minute", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 2.0);

        block.run(56.0); // Started at seconds = 3
        assert_eq!(block.data.scalar(), 2.0);

        block.run(57.0);
        assert_eq!(block.data.scalar(), 3.0);
    }

    #[test]
    fn test_system_time_block_hour() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 7, 27, 1, 2, 3)
            .unwrap();
        let mut block = SystemTimeBlock::new("SystemTime1", "Hour", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 1.0);

        block.run(57.0 * 60.0 + 56.0); // Started at minute 2, second = 3
        assert_eq!(block.data.scalar(), 1.0);

        block.run(57.0 * 60.0 + 57.0);
        assert_eq!(block.data.scalar(), 2.0);
    }

    #[test]
    fn test_system_time_block_day_lunar() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 7, 31, 1, 2, 3)
            .unwrap();
        let mut block = SystemTimeBlock::new("SystemTime1", "DayLunar", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 31.0);

        // 22 hours later it's still the 31st
        block.run(60.0 * 60.0 * 22.0);
        assert_eq!(block.data.scalar(), 31.0);

        // Rolls over to the first
        block.run(60.0 * 60.0 * 23.0);
        assert_eq!(block.data.scalar(), 1.0);
    }

    #[test]
    fn test_system_time_block_day_ordinal() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 12, 31, 1, 2, 3)
            .unwrap();
        let mut block =
            SystemTimeBlock::new("SystemTime1", "DayOrdinal", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 365.0);

        // 22 hours later it's still the last day of the year
        block.run(60.0 * 60.0 * 22.0);
        assert_eq!(block.data.scalar(), 365.0);

        // Rolls over to the first day of new year
        block.run(60.0 * 60.0 * 23.0);
        assert_eq!(block.data.scalar(), 1.0);
    }

    #[test]
    fn test_system_time_block_day_month() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 12, 31, 1, 2, 3)
            .unwrap();
        let mut block = SystemTimeBlock::new("SystemTime1", "Month", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 12.0);

        // 22 hours later it's still the last day of the year
        block.run(60.0 * 60.0 * 22.0);
        assert_eq!(block.data.scalar(), 12.0);

        // Rolls over to the first day of new year
        block.run(60.0 * 60.0 * 23.0);
        assert_eq!(block.data.scalar(), 1.0);
    }

    #[test]
    fn test_system_time_block_day_year() {
        use chrono::TimeZone;
        let system_time_at_app_start_s = chrono::offset::Local
            .with_ymd_and_hms(2022, 12, 31, 1, 2, 3)
            .unwrap();
        let mut block = SystemTimeBlock::new("SystemTime1", "Year", system_time_at_app_start_s);

        block.run(0.0);
        assert_eq!(block.data.scalar(), 2022.0);

        // 22 hours later it's still the last day of the year
        block.run(60.0 * 60.0 * 22.0);
        assert_eq!(block.data.scalar(), 2022.0);

        // Rolls over to the first day of new year
        block.run(60.0 * 60.0 * 23.0);
        assert_eq!(block.data.scalar(), 2023.0);
    }
}
