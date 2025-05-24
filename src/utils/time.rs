use chrono::{DateTime, Utc, TimeZone, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前UTC时间戳（秒）
    pub fn now_timestamp() -> i64 {
        Utc::now().timestamp()
    }
    
    /// 获取当前UTC时间戳（毫秒）
    pub fn now_timestamp_millis() -> i64 {
        Utc::now().timestamp_millis()
    }
    
    /// 获取当前UTC时间
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// 获取当前本地时间
    pub fn now_local() -> DateTime<Local> {
        Local::now()
    }
    
    /// 时间戳转UTC时间
    pub fn timestamp_to_utc(timestamp: i64) -> Option<DateTime<Utc>> {
        Utc.timestamp_opt(timestamp, 0).single()
    }
    
    /// 时间戳转本地时间
    pub fn timestamp_to_local(timestamp: i64) -> Option<DateTime<Local>> {
        Local.timestamp_opt(timestamp, 0).single()
    }
    
    /// 格式化时间为字符串
    pub fn format_datetime(dt: &DateTime<Utc>, format: &str) -> String {
        dt.format(format).to_string()
    }
    
    /// 解析时间字符串
    pub fn parse_datetime(s: &str, format: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        let naive = NaiveDateTime::parse_from_str(s, format)?;
        Ok(Utc.from_utc_datetime(&naive))
    }
    
    /// 获取今天开始时间（UTC）
    pub fn today_start_utc() -> DateTime<Utc> {
        let now = Utc::now();
        now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
    }
    
    /// 获取今天结束时间（UTC）
    pub fn today_end_utc() -> DateTime<Utc> {
        let now = Utc::now();
        now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc()
    }
    
    /// 获取指定天数前的时间
    pub fn days_ago(days: i64) -> DateTime<Utc> {
        Utc::now() - chrono::Duration::days(days)
    }
    
    /// 获取指定小时前的时间
    pub fn hours_ago(hours: i64) -> DateTime<Utc> {
        Utc::now() - chrono::Duration::hours(hours)
    }
    
    /// 获取指定分钟前的时间
    pub fn minutes_ago(minutes: i64) -> DateTime<Utc> {
        Utc::now() - chrono::Duration::minutes(minutes)
    }
    
    /// 检查时间是否在指定范围内
    pub fn is_within_range(
        time: &DateTime<Utc>,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> bool {
        time >= start && time <= end
    }
    
    /// 计算两个时间的差值（秒）
    pub fn diff_seconds(time1: &DateTime<Utc>, time2: &DateTime<Utc>) -> i64 {
        (time1.timestamp() - time2.timestamp()).abs()
    }
}

/// 时间相关的常量
pub mod constants {
    pub const SECONDS_PER_MINUTE: i64 = 60;
    pub const SECONDS_PER_HOUR: i64 = 3600;
    pub const SECONDS_PER_DAY: i64 = 86400;
    pub const SECONDS_PER_WEEK: i64 = 604800;
    
    pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    pub const DATE_FORMAT: &str = "%Y-%m-%d";
    pub const TIME_FORMAT: &str = "%H:%M:%S";
}

/// 时间段枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePeriod {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "24h")]
    TwentyFourHours,
}

impl TimePeriod {
    pub fn to_seconds(&self) -> i64 {
        match self {
            TimePeriod::OneMinute => 60,
            TimePeriod::FiveMinutes => 300,
            TimePeriod::OneHour => 3600,
            TimePeriod::SixHours => 21600,
            TimePeriod::TwentyFourHours => 86400,
        }
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            TimePeriod::OneMinute => "1m",
            TimePeriod::FiveMinutes => "5m",
            TimePeriod::OneHour => "1h",
            TimePeriod::SixHours => "6h",
            TimePeriod::TwentyFourHours => "24h",
        }
    }
}
