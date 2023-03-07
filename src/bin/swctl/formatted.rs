use std::time::Duration;

use chrono::{Local, DateTime, NaiveTime};
use stopwatchd::{
    communication::details::StopwatchDetails,
    util::get_uuid_node
};
use tabled::{Tabled, Table, builder::Builder};

pub const DEFAULT_DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_DURATION_FORMAT: &'static str = "%H:%M:%S.%3f";

const BDHS: usize = 7;
pub const BASIC_DETAILS_HEADERS_SHOWDT: [&'static str; BDHS] = [
    "id",
    "name",
    "state",
    "start time", // index 3
    "total time",
    "laps count",
    "lap time"
];

const BDHN: usize = 6;
pub const BASIC_DETAILS_HEADERS_NODT: [&'static str; BDHN] = [
    "id",
    "name",
    "state",
    "total time",
    "laps count",
    "lap time"
];

#[derive(Tabled, Clone, Debug)]
pub struct BasicStopwatchDetails {
    pub id: String,
    pub name: String,
    pub state: String,
    pub start_time: String,
    pub total_time: String,
    pub laps_count: String,
    pub current_lap_time: String
}

impl BasicStopwatchDetails {
    pub fn to_table<I>(bsd_iter: I, show_dt: bool) -> Table
    where
        I: IntoIterator<Item = BasicStopwatchDetails>
    {
        let mut builder = Builder::default();
        if show_dt {
            Self::to_table_with_dt(&mut builder, bsd_iter);
        } else {
            Self::to_table_no_dt(&mut builder, bsd_iter);
        }
        builder.build()
    }

    fn to_table_with_dt<I>(builder: &mut Builder, bsd_iter: I)
    where
        I: IntoIterator<Item = BasicStopwatchDetails>
    {
        builder.set_columns(BASIC_DETAILS_HEADERS_SHOWDT);
        for bsd in bsd_iter {
            let record: [String; BDHS] = [
                bsd.id,
                bsd.name,
                bsd.state,
                bsd.start_time,
                bsd.total_time,
                bsd.laps_count,
                bsd.current_lap_time
            ];
            builder.add_record(record);
        }
    }

    fn to_table_no_dt<I>(builder: &mut Builder, bsd_iter: I)
    where
        I: IntoIterator<Item = BasicStopwatchDetails>
    {
        builder.set_columns(BASIC_DETAILS_HEADERS_NODT);
        for bsd in bsd_iter {
            let record: [String; BDHN] = [
                bsd.id,
                bsd.name,
                bsd.state,
                bsd.total_time,
                bsd.laps_count,
                bsd.current_lap_time
            ];
            builder.add_record(record);
        }
    }
}

pub struct BasicStopwatchDetailsBuilder {
    pub datetime_format: String,
    pub duration_format: String
}

impl BasicStopwatchDetailsBuilder {
    pub fn new(datetime_format: &str, duration_format: &str) -> Self {
        let datetime_format = datetime_format.to_string();
        let duration_format = duration_format.to_string();
        Self { datetime_format, duration_format }
    }

    pub fn format_datetime<T>(&self, time: T) -> String
    where
        T: Into<DateTime<Local>>
    {
        time.into().format(&self.datetime_format).to_string()
    }

    pub fn format_duration<D>(&self, duration: D) -> String
    where
        D: Into<NaiveTime>
    {
        let time = duration.into();
        time.format(&self.duration_format).to_string()
    }

    pub fn get_details(&self, details: StopwatchDetails, show_dt: bool) -> BasicStopwatchDetails {
        let id = format!("{:x}", get_uuid_node(&details.sw_id));
        let name = details.name.to_string();
        let state = format!("{}", details.state);
        let start_time = if show_dt {
            details.start_time
                .map(|st| self.format_datetime(st))
                .unwrap_or("none".to_string())
        } else {
            String::new()
        };
        let total_time = self.format_duration(std_duration_to_naive(details.total_time));
        let laps_count = format!("{}", details.laps_count());
        let current_lap_time = self.format_duration(
            std_duration_to_naive(details.current_lap_time())
        );
        BasicStopwatchDetails {
            id,
            name,
            state,
            start_time,
            total_time,
            laps_count,
            current_lap_time
        }
    }
}

pub fn std_duration_to_naive(duration: Duration) -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        + chrono::Duration::from_std(duration).unwrap_or_else(|_| chrono::Duration::max_value())

}