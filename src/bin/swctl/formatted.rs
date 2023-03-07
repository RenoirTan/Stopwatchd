use std::time::Duration;

use chrono::{Local, DateTime, NaiveTime};
use stopwatchd::{
    communication::details::StopwatchDetails,
    util::get_uuid_node, models::lap::FinishedLap
};
use tabled::{Tabled, Table, builder::Builder};
use uuid::Uuid;

pub const DEFAULT_DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_DURATION_FORMAT: &'static str = "%H:%M:%S.%3f";

pub struct DetailsBuilder {
    pub datetime_format: String,
    pub duration_format: String
}

impl DetailsBuilder {
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

    pub fn get_basic(&self, details: StopwatchDetails, show_dt: bool) -> BasicStopwatchDetails {
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

    pub fn get_verbose_lap(&self, lap: FinishedLap, show_dt: bool) -> VerboseLap {
        let id = lap.id
            .as_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();
        let sw_id = format!("{:x}", get_uuid_node(&lap.sw_id));
        let start = if show_dt {
            self.format_datetime(lap.start)
        } else {
            String::new()
        };
        let duration = self.format_duration(std_duration_to_naive(lap.duration));
        VerboseLap { id, sw_id, start, duration }
    }
}

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
    pub fn to_array_with_dt(self) -> [String; BDHS] {
        [
            self.id,
            self.name,
            self.state,
            self.start_time,
            self.total_time,
            self.laps_count,
            self.current_lap_time
        ]
    }

    pub fn to_array_no_dt(self) -> [String; BDHN] {
        [
            self.id,
            self.name,
            self.state,
            self.total_time,
            self.laps_count,
            self.current_lap_time
        ]
    }

    pub fn add_to_builder(self, show_dt: bool, builder: &mut Builder) {
        if show_dt {
            let record = self.to_array_with_dt();
            for (bdhs_header, field) in BASIC_DETAILS_HEADERS_SHOWDT.into_iter().zip(record) {
                builder.add_record([bdhs_header.to_string(), field]);
            }
        } else {
            let record = self.to_array_no_dt();
            for (bdhn_header, field) in BASIC_DETAILS_HEADERS_NODT.into_iter().zip(record) {
                builder.add_record([bdhn_header.to_string(), field]);
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_record(self, show_dt: bool) -> Table {
        let mut builder = Builder::default();
        self.add_to_builder(show_dt, &mut builder);
        builder.build()
    }

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
            let record = bsd.to_array_with_dt();
            builder.add_record(record);
        }
    }

    fn to_table_no_dt<I>(builder: &mut Builder, bsd_iter: I)
    where
        I: IntoIterator<Item = BasicStopwatchDetails>
    {
        builder.set_columns(BASIC_DETAILS_HEADERS_NODT);
        for bsd in bsd_iter {
            let record = bsd.to_array_no_dt();
            builder.add_record(record);
        }
    }
}

const VLHS: usize = 4;
pub const VERBOSELAP_HEADERS_SHOWDT: [&'static str; VLHS] = [
    "id",
    "stopwatch id",
    "start",
    "duration"
];

const VLHN: usize = 3;
pub const VERBOSELAP_HEADERS_NODT: [&'static str; VLHN] = [
    "id",
    "stopwatch id",
    "duration"
];

#[derive(Tabled, Clone, Debug)]
pub struct VerboseLap {
    pub id: String,
    pub sw_id: String,
    pub start: String,
    pub duration: String
}

impl VerboseLap {
    pub fn to_array_with_dt(self) -> [String; VLHS] {
        [
            self.id,
            self.sw_id,
            self.start,
            self.duration
        ]
    }

    pub fn to_array_no_dt(self) -> [String; VLHN] {
        [
            self.id,
            self.sw_id,
            self.duration
        ]
    }

    pub fn add_to_builder(self, builder: &mut Builder, show_dt: bool) {
        if show_dt {
            builder.add_record(self.to_array_with_dt());
        } else {
            builder.add_record(self.to_array_no_dt());
        }
    }
}

pub struct VerboseDetails {
    pub basic: BasicStopwatchDetails,
    pub verbose: Vec<VerboseLap>
}

impl VerboseDetails {
    pub fn from_details(
        details: StopwatchDetails,
        show_dt: bool,
        builder: &DetailsBuilder
    ) -> Self {
        let verbose = match details.verbose_info {
            Some(ref vi) => {
                vi.laps.iter().map(|l| builder.get_verbose_lap(l.clone(), show_dt)).collect()
            },
            None => vec![]
        };
        let basic = builder.get_basic(details, show_dt);
        Self { basic, verbose }
    }
    
    pub fn add_to_builder(
        self,
        show_dt: bool,
        basic_builder: &mut Builder,
        verbose_builder: &mut Builder
    ) {
        self.basic.add_to_builder(show_dt, basic_builder);
        for lap in self.verbose {
            lap.add_to_builder(verbose_builder, show_dt);
        }
    }

    pub fn to_basic_and_verbose(self, show_dt: bool) -> (Table, Table) {
        let mut basic_builder = Builder::default();
        let mut verbose_builder = Builder::default();
        if show_dt {
            verbose_builder.set_columns(VERBOSELAP_HEADERS_SHOWDT);
        } else {
            verbose_builder.set_columns(VERBOSELAP_HEADERS_NODT);
        }
        self.add_to_builder(show_dt, &mut basic_builder, &mut verbose_builder);
        (basic_builder.build(), verbose_builder.build())
    }
}

pub fn std_duration_to_naive(duration: Duration) -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        + chrono::Duration::from_std(duration).unwrap_or_else(|_| chrono::Duration::max_value())

}