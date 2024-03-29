use std::time::Duration;

use chrono::{Local, DateTime, NaiveTime};

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

    /// Format a [`NaiveTime`] object into a [`String`].
    pub fn format_naive_time<D>(&self, duration: D) -> String
    where
        D: Into<NaiveTime>
    {
        let time = duration.into();
        time.format(&self.duration_format).to_string()
    }

    /// Format a [`Duration`] object into a [`String`].
    pub fn format_duration<D>(&self, duration: D) -> String
    where
        D: Into<Duration>
    {
        // TODO: Unfix this format
        let duration: Duration = duration.into();
        let seconds = duration.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds/60) % 60;
        let seconds = seconds % 60;
        let milliseconds = duration.as_millis() % 1000;
        let s = format!("{}:{}:{}.{}", hours, minutes, seconds, milliseconds);
        // let s = std_duration_to_naive(duration.into()).format(&self.duration_format).to_string();
        s
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new(DEFAULT_DATETIME_FORMAT, DEFAULT_DURATION_FORMAT)
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
