use chrono::{DateTime, Local};
use clap::ValueEnum;
use colored::*;

// Define an enum for log levels
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Debug,
    Info,
    Success,
    Warning,
    Error,
}

// Define a logger struct
pub struct Logger {
    min_log_level: LogLevel, // Minimum log level to display
    no_log: bool,            // Whether to suppress logs
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            min_log_level: LogLevel::Info, // Default minimum log level is Info
            no_log: false,                 // Default to showing logs
        }
    }
}

impl Logger {
    // Constructor to create a new Logger with optional minimum log level and no_log flag
    pub fn new(min_log_level: Option<LogLevel>, no_log: Option<bool>) -> Self {
        Logger {
            min_log_level: min_log_level.unwrap_or(LogLevel::Info), // Default to Info if None
            no_log: no_log.unwrap_or(false),                        // Default to false if None
        }
    }

    // Function to get the current timestamp
    fn get_timestamp() -> DateTime<Local> {
        Local::now()
    }

    // Function to format the timestamp as a string
    fn format_timestamp(timestamp: DateTime<Local>) -> String {
        // Format the timestamp with year-month-day hour:minute:second am/pm
        timestamp.format("%Y-%m-%d %I:%M:%S %p").to_string()
    }

    // Function to apply color to the log message based on log level
    fn colored_log(&self, level: LogLevel, message: &str) {
        // Check if logging is disabled
        if self.no_log && level != LogLevel::Error {
            return;
        }

        // Check if the log level is below the minimum log level
        if level < self.min_log_level {
            return; // Ignore logs below the minimum level
        }

        // Get the current timestamp
        let timestamp = Self::get_timestamp();
        // Format the timestamp as a string
        let formatted_timestamp = Self::format_timestamp(timestamp);

        // Create a formatted log message including timestamp, level, and the actual message
        let formatted_message = format!("{} [{:?}]: {}", formatted_timestamp, level, message);

        // Print the log message with color based on the log level
        match level {
            LogLevel::Success => println!("{}", formatted_message.bright_green()), // Apply bright green color to success logs
            LogLevel::Error => println!("{}", formatted_message.bright_red()), // Apply bright red color to error logs
            LogLevel::Warning => println!("{}", formatted_message.bright_yellow()), // Apply bright yellow color to warning logs
            LogLevel::Info => println!("{}", formatted_message.bright_cyan()), // Apply bright cyan color to info logs
            LogLevel::Debug => println!("{}", formatted_message.bright_blue()), // Apply bright blue color to debug logs
        }
    }

    // Convenience functions for different log levels that call colored_log with their respective log level
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
