use stopwatchd::{
    communication::details::StopwatchDetails,
    util::get_uuid_node
};
use tabled::Tabled;
use time::{OffsetDateTime, format_description::{self, FormatItem}};

pub const DEFAULT_TIME_FORMAT: &'static str = "";

#[derive(Tabled, Clone, Debug)]
pub struct BasicStopwatchDetails {
    pub id: String,
    pub name: String,
    pub state: String,
    pub start_time: String,
    pub total_time: String,
    pub laps_count: String
}

pub struct BasicStopwatchDetailsBuilder<'a> {
    pub time_format: Vec<FormatItem<'a>> // TODO: Create time formatter
}

impl<'a> BasicStopwatchDetailsBuilder<'a> {
    pub fn new(time_format: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let time_format = format_description::parse(time_format).map_err(Box::new)?;
        Ok(Self { time_format })
    }

    pub fn format_time<T>(&self, time: T) -> Result<String, time::error::Format>
    where
        T: Into<OffsetDateTime>
    {
        time.into().format(&self.time_format)
    }

    pub fn get_details(&self, details: StopwatchDetails) -> BasicStopwatchDetails {
        let id = format!("{:x}", get_uuid_node(&details.sw_id));
        let name = details.name.to_string();
        let state = format!("{}", details.state);
        let start_time = match details.start_time {
            Some(st) => self.format_time(st).unwrap_or("bad format".to_string()),
            None => "none".to_string()
        };
        // TODO: Find a way to allow custom formatting of total_time
        let total_time = format!("{} ms", details.total_time.as_millis());
        let laps_count = format!("{}", details.laps_count());
        BasicStopwatchDetails { id, name, state, start_time, total_time, laps_count }
    }
}