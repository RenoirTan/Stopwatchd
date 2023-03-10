use std::time::Duration;

use chrono::{Local, DateTime, NaiveTime};
use stopwatchd::{
    communication::details::StopwatchDetails,
    util::get_uuid_node, models::lap::FinishedLap
};
use tabled::builder::Builder;
use uuid::Uuid;

pub const DEFAULT_DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_DURATION_FORMAT: &'static str = "%H:%M:%S.%3f";

pub struct Formatter {
    pub datetime_format: String,
    pub duration_format: String
}

impl Formatter {
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

    pub fn get_basic(&self, details: StopwatchDetails, show_dt: bool) -> BasicRecord {
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

        [
            id,
            name,
            state,
            start_time,
            total_time,
            laps_count,
            current_lap_time
        ]
    }

    pub fn get_verbose_lap(&self, lap: FinishedLap, show_dt: bool) -> VerboseRecord {
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
        [ id, sw_id, start, duration ]
    }

    pub fn from_details<I>(&self, builder: &mut Builder, details: I, show_dt: bool)
    where
        I: IntoIterator<Item = StopwatchDetails>
    {
        for d in details {
            let record = self.get_basic(d, show_dt);
            add_basic_record_to_builder(builder, record, show_dt);
        }
    }

    pub fn from_details_verbose<'s, I>(
        &'s self,
        basic: Builder<'s>,
        verbose: Builder<'s>,
        details: I,
        show_dt: bool
    ) -> impl Iterator<Item = (Builder, Builder)>
    where
        I: IntoIterator<Item = StopwatchDetails> + 's
    {
        details.into_iter().map(move |d| {
            let mut basic = basic.clone();
            let mut verbose = verbose.clone();
            if let Some(ref vi) = d.verbose_info {
                for lap in &vi.laps {
                    let lap = lap.clone();
                    let record = self.get_verbose_lap(lap, show_dt);
                    add_verbose_record_to_builder(&mut verbose, record, show_dt);
                }
            }
            let basic_record = self.get_basic(d, show_dt);
            add_basic_to_verbose_builder(&mut basic, basic_record, show_dt);
            (basic, verbose)
        })
    }
}

pub type BasicRecord = [String; BDH_COUNT];
pub const BDH_COUNT: usize = 7;
pub const BASIC_DETAILS_HEADERS: [&'static str; BDH_COUNT] = [
    "id",
    "name",
    "state",
    "start time", // index 3
    "total time",
    "laps count",
    "lap time"
];
pub const BDH_SDI_COUNT: usize = 1;
pub const BDH_SHOWDT_INDICES: [usize; BDH_SDI_COUNT] = [3];

pub type VerboseRecord = [String; VLH_COUNT];
pub const VLH_COUNT: usize = 4;
pub const VERBOSELAP_HEADERS: [&'static str; VLH_COUNT] = [
    "id",
    "stopwatch id",
    "start", // index 2
    "duration"
];
pub const VLH_SDI_COUNT: usize = 1;
pub const VLH_SHOWDT_INDICES: [usize; VLH_SDI_COUNT] = [2];

pub fn std_duration_to_naive(duration: Duration) -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        + chrono::Duration::from_std(duration).unwrap_or_else(|_| chrono::Duration::max_value())
}

pub fn get_basic_table_builder<'b>(show_dt: bool) -> Builder<'b> {
    let mut builder = Builder::default();
    if show_dt {
        builder.set_columns(BASIC_DETAILS_HEADERS);
    } else {
        builder.set_columns(all_except_indices(BASIC_DETAILS_HEADERS, &BDH_SHOWDT_INDICES));
    }
    builder
}

pub fn get_basic_single_builder<'b>(_show_dt: bool) -> Builder<'b> {
    let builder = Builder::default();
    builder
}

pub fn get_verbose_table_builder<'b>(show_dt: bool) -> Builder<'b> {
    let mut builder = Builder::default();
    if show_dt {
        builder.set_columns(VERBOSELAP_HEADERS);
    } else {
        builder.set_columns(all_except_indices(VERBOSELAP_HEADERS, &VLH_SHOWDT_INDICES));
    }
    builder
}

pub fn add_basic_record_to_builder(builder: &mut Builder, record: BasicRecord, show_dt: bool) {
    if show_dt {
        builder.add_record(record);
    } else {
        builder.add_record(all_except_indices(record, &BDH_SHOWDT_INDICES));
    }
}

pub fn add_basic_to_verbose_builder(builder: &mut Builder, record: BasicRecord, show_dt: bool) {
    let iter = if show_dt {
        all_except_indices(0..BDH_COUNT, &[])
    } else {
        all_except_indices(0..BDH_COUNT, &BDH_SHOWDT_INDICES)
    };
    for index in iter {
        let header = BASIC_DETAILS_HEADERS[index].to_string();
        let field = record[index].clone();
        builder.add_record([header, field]);
    }
}

pub fn add_verbose_record_to_builder(builder: &mut Builder, record: VerboseRecord, show_dt: bool) {
    if show_dt {
        builder.add_record(record);
    } else {
        builder.add_record(all_except_indices(record, &VLH_SHOWDT_INDICES));
    }
}

fn all_except_indices<'i, I, V>(iter: I, indices: &'i [usize]) -> impl Iterator<Item = V> + 'i
where
    I: IntoIterator<Item = V> + 'i
{
    iter.into_iter()
        .enumerate()
        .filter_map(|(i, v)| if indices.contains(&i) { None } else { Some(v) })
}