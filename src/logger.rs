use chrono::{DateTime, Local};
use clap::ValueEnum;
use owo_colors::OwoColorize;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Debug,
    Info,
    Success,
    Warning,
    Error,
}

pub struct Logger {
    min_log_level: LogLevel,
    no_log: bool,
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            min_log_level: LogLevel::Info,
            no_log: false,
        }
    }
}

impl Logger {
    pub fn new(min_log_level: Option<LogLevel>, no_log: Option<bool>) -> Self {
        Logger {
            min_log_level: min_log_level.unwrap_or(LogLevel::Info),
            no_log: no_log.unwrap_or(false),
        }
    }

    fn get_timestamp() -> DateTime<Local> {
        Local::now()
    }

    fn format_timestamp(timestamp: DateTime<Local>) -> String {
        timestamp.format("%H:%M:%S").to_string()
    }

    fn colored_log(&self, level: LogLevel, message: &str) {
        if self.no_log && level != LogLevel::Error {
            return;
        }

        if level < self.min_log_level {
            return;
        }

        let ts = Self::format_timestamp(Self::get_timestamp());
        let (label, rgb): (&str, (u8, u8, u8)) = match level {
            LogLevel::Debug => ("DEBUG  ", (148, 163, 184)),
            LogLevel::Info => ("INFO   ", (56, 189, 248)),
            LogLevel::Success => ("SUCCESS", (16, 185, 129)),
            LogLevel::Warning => ("WARN   ", (245, 158, 11)),
            LogLevel::Error => ("ERROR  ", (239, 68, 68)),
        };

        let ts_styled = format!("\u{2502} {} \u{2502}", ts);
        let label_styled = label.truecolor(rgb.0, rgb.1, rgb.2).bold().to_string();

        let message_styled = match level {
            LogLevel::Error => message.truecolor(252, 165, 165).to_string(),
            LogLevel::Warning => message.truecolor(253, 224, 71).to_string(),
            LogLevel::Success => message.truecolor(134, 239, 172).to_string(),
            LogLevel::Debug => message.dimmed().to_string(),
            LogLevel::Info => message.to_string(),
        };

        println!(
            "{} {}  {}",
            ts_styled.dimmed(),
            label_styled,
            message_styled
        );
    }

    pub fn success(&self, message: &str) {
        self.colored_log(LogLevel::Success, message);
    }

    pub fn error(&self, message: &str) {
        self.colored_log(LogLevel::Error, message);
    }

    pub fn warning(&self, message: &str) {
        self.colored_log(LogLevel::Warning, message);
    }

    pub fn info(&self, message: &str) {
        self.colored_log(LogLevel::Info, message);
    }

    pub fn debug(&self, message: &str) {
        self.colored_log(LogLevel::Debug, message);
    }
}
