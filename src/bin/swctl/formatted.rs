//! Format the [`StopwatchDetails`] returned from `swd` into a printable format.

use std::{time::Duration, fmt};

use chrono::{Local, DateTime, NaiveTime};
use clap::ValueEnum;
use stopwatchd::{
    communication::{details::StopwatchDetails, server::ServerError},
    util::get_uuid_node,
    models::lap::FinishedLap,
    identifiers::Identifier
};
use tabled::{Table, Tabled, settings::Style};
use uuid::Uuid;

/// Table styles. See [`tabled`] for more information.
#[derive(Copy, Clone, Debug, Default, ValueEnum)]
#[non_exhaustive]
pub enum Styles {
    #[default] Blank,
    Empty,
    Ascii,
    AsciiRounded,
    Psql,
    Markdown,
    Modern,
    Sharp,
    Rounded,
    Extended,
    Dots,
    Rest // RestructuredText
}

impl Styles {
    /// Modify a [`Table`]'s style.
    pub fn style_table(self, table: &mut Table) {
        use Styles::*;
        match self {
            Blank => table.with(Style::blank()),
            Empty => table.with(Style::empty()),
            Ascii => table.with(Style::ascii()),
            AsciiRounded => table.with(Style::ascii_rounded()),
            Psql => table.with(Style::psql()),
            Markdown => table.with(Style::markdown()),
            Modern => table.with(Style::modern()),
            Sharp => table.with(Style::sharp()),
            Rounded => table.with(Style::rounded()),
            Extended => table.with(Style::extended()),
            Dots => table.with(Style::dots()),
            Rest => table.with(Style::re_structured_text())
        };
    }
}

impl fmt::Display for Styles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Styles::*;
        write!(f, "{}", match self {
            Blank => "blank",
            Empty => "empty",
            Ascii => "ascii",
            AsciiRounded => "ascii_rounded",
            Psql => "psql",
            Markdown => "markdown",
            Modern => "modern",
            Sharp => "sharp",
            Rounded => "rounded",
            Extended => "extended",
            Dots => "dots",
            Rest => "rest"
        })
    }
}

/// Format for date and time.
pub const DEFAULT_DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
/// Format for duration.
pub const DEFAULT_DURATION_FORMAT: &'static str = "%H:%M:%S.%3f";

/// Formats details for printing.
pub struct Formatter {
    pub datetime_format: String,
    pub duration_format: String
}

impl Formatter {
    /// Create a new formatter.
    /// 
    /// `datetime_format` and `duration_format` use C's strftime's format
    /// specifiers.
    /// 
    /// You can use [`DEFAULT_DATETIME_FORMAT`] and [`DEFAULT_DURATION_FORMAT`]
    /// as defaults if the user doesn't specify.
    pub fn new(datetime_format: &str, duration_format: &str) -> Self {
        let datetime_format = datetime_format.to_string();
        let duration_format = duration_format.to_string();
        Self { datetime_format, duration_format }
    }

    /// Format a date and time object into a [`String`].
    pub fn format_datetime<T>(&self, time: T) -> String
    where
        T: Into<DateTime<Local>>
    {
        time.into().format(&self.datetime_format).to_string()
    }

    /// Format a duration object into a [`String`].
    pub fn format_duration<D>(&self, duration: D) -> String
    where
        D: Into<NaiveTime>
    {
        let time = duration.into();
        time.format(&self.duration_format).to_string()
    }
}

/// Record of non-verbose parts of [`StopwatchDetails`].
#[derive(Tabled, Clone, Debug, PartialEq, Eq)]
pub struct BasicDetails {
    #[tabled(rename = "id")] pub id: String,
    #[tabled(rename = "name")] pub name: String,
    #[tabled(rename = "state")] pub state: String,
    #[tabled(rename = "start time")] pub start_time: String,
    #[tabled(rename = "total duration")] pub total_time: String,
    #[tabled(rename = "laps count")] pub laps_count: String,
    #[tabled(rename = "current lap time")] pub current_lap_time: String
}

impl BasicDetails {
    /// Format basic fields from [`StopwatchDetails`] into human-readable
    /// strings.
    /// 
    /// # See Also
    /// [`BasicDetailsNoDT`]. Set `show_dt` to [`false`].
    pub fn format(formatter: &Formatter, details: &StopwatchDetails, show_dt: bool) -> Self {
        let id = format!("{:x}", get_uuid_node(&details.sw_id));
        let name = details.name.to_string();
        let state = format!("{}", details.state);
        let start_time = if show_dt {
            details.start_time
                .map(|st| formatter.format_datetime(st))
                .unwrap_or("none".to_string())
        } else {
            String::new()
        };
        let total_time = formatter.format_duration(std_duration_to_naive(details.total_time));
        let laps_count = format!("{}", details.laps_count());
        let current_lap_time = formatter.format_duration(
            std_duration_to_naive(details.current_lap_time())
        );
        Self { id, name, state, start_time, total_time, laps_count, current_lap_time }
    }
}

/// Like [`BasicDetails`] but without fields that carry date and time
/// information. Converts from [`BasicDetails`] using [`From::from`]
/// by discarding unneeded data.
#[derive(Tabled, Clone, Debug, PartialEq, Eq)]
pub struct BasicDetailsNoDT {
    #[tabled(rename = "id")] pub id: String,
    #[tabled(rename = "name")] pub name: String,
    #[tabled(rename = "state")] pub state: String,
    #[tabled(rename = "total duration")] pub total_time: String,
    #[tabled(rename = "laps count")] pub laps_count: String,
    #[tabled(rename = "current lap time")] pub current_lap_time: String
}

impl From<BasicDetails> for BasicDetailsNoDT {
    fn from(value: BasicDetails) -> Self {
        Self {
            id: value.id,
            name: value.name,
            state: value.state,
            total_time: value.total_time,
            laps_count: value.laps_count,
            current_lap_time: value.current_lap_time
        }
    }
}

/// Verbose information about each lap from [`StopwatchDetails`].
#[derive(Tabled, Clone, Debug, PartialEq, Eq)]
pub struct VerboseDetails {
    #[tabled(rename = "id")] pub id: String,
    #[tabled(rename = "stopwatch id")] pub stopwatch_id: String,
    #[tabled(rename = "start time")] pub start: String,
    #[tabled(rename = "duration")] pub duration: String
}

impl VerboseDetails {
    /// Convert each lap into [`VerboseDetails`].
    pub fn format(formatter: &Formatter, lap: &FinishedLap, show_dt: bool) -> Self {
        let id = lap.id
            .as_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();
        let stopwatch_id = format!("{:x}", get_uuid_node(&lap.sw_id));
        let start = if show_dt {
            formatter.format_datetime(lap.start)
        } else {
            String::new()
        };
        let duration = formatter.format_duration(std_duration_to_naive(lap.duration));
        Self { id, stopwatch_id, start, duration }
    }
}

/// [`VerboseDetails`] but with no fields containing date and time information.
#[derive(Tabled, Clone, Debug, PartialEq, Eq)]
pub struct VerboseDetailsNoDT {
    #[tabled(rename = "id")] pub id: String,
    #[tabled(rename = "stopwatch id")] pub stopwatch_id: String,
    #[tabled(rename = "duration")] pub duration: String
}

impl From<VerboseDetails> for VerboseDetailsNoDT {
    fn from(value: VerboseDetails) -> Self {
        Self {
            id: value.id,
            stopwatch_id: value.stopwatch_id,
            duration: value.duration
        }
    }
}

/// Formatted [`ServerError`] thrown by `swd`.
#[derive(Tabled, Clone, Debug, PartialEq, Eq)]
pub struct ErrorRecord {
    #[tabled(rename = "identifier")] pub identifier: String,
    #[tabled(rename = "message")] pub message: String
}

impl ErrorRecord {
    /// Convert [`ServerError`] into human-readable text and associate it with
    /// a corresponding stopwatch [`Identifier`]. If the parameter `identifier`
    /// is [`None`], the error is associated with an error with `swd` or `swctl`
    /// and not a particular in stopwatch.
    pub fn format(
        _formatter: &Formatter,
        identifier: Option<&Identifier>,
        error: ServerError
    ) -> Self {
        let identifier = identifier
            .map(|i| i.to_string())
            .unwrap_or_else(|| "<SYSTEM>".to_string());
        let message = format!("{}", error);
        Self { identifier, message }
    }
}

/// Convert a [`Duration`] into [`NaiveTime`]. Since [`Duration`] only stores
/// days, minutes, seconds and smaller units, it must be converted into a format
/// that stores larger units like days and months like [`NaiveTime`] for
/// display.
pub fn std_duration_to_naive(duration: Duration) -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        + chrono::Duration::from_std(duration).unwrap_or_else(|_| chrono::Duration::max_value())
}

/// Iterate through all items except for those at the specified indices.
/// Indices start from 0 as per normal.
#[warn(deprecated)]
#[allow(unused)]
fn all_except_indices<'i, I, V>(iter: I, indices: &'i [usize]) -> impl Iterator<Item = V> + 'i
where
    I: IntoIterator<Item = V> + 'i
{
    iter.into_iter()
        .enumerate()
        .filter_map(|(i, v)| if indices.contains(&i) { None } else { Some(v) })
}