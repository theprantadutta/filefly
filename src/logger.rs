use chrono::{DateTime, Local};
use colored::*;

// Define a logger struct
pub struct Logger;

impl Logger {
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
    fn colored_log(&self, level: &str, message: &str) {
        // Get the current timestamp
        let timestamp = Self::get_timestamp();
        // Format the timestamp as a string
        let formatted_timestamp = Self::format_timestamp(timestamp);

        // Create a formatted log message including timestamp, level, and the actual message
        let formatted_message = format!("{} [{}]: {}", formatted_timestamp, level, message);

        // Print the log message with color based on the log level
        match level {
            "SUCCESS" => println!("{}", formatted_message.bright_green()), // Apply bright green color to success logs
            "ERROR" => println!("{}", formatted_message.bright_red()), // Apply bright red color to error logs
            "WARNING" => println!("{}", formatted_message.bright_yellow()), // Apply bright yellow color to warning logs
            "INFO" => println!("{}", formatted_message.bright_cyan()), // Apply bright cyan color to info logs
            "DEBUG" => println!("{}", formatted_message.bright_blue()), // Apply bright blue color to debug logs
            _ => println!("{}", formatted_message), // Default color for unrecognized log levels
        }
    }

    // Log function that calls colored_log with a log level of "LOG"
    // pub fn log(&self, message: &str) {
    //     self.colored_log("LOG", message);
    // }

    // Convenience functions for different log levels that call colored_log with their respective log level
    pub fn success(&self, message: &str) {
        self.colored_log("SUCCESS", message);
    }

    pub fn error(&self, message: &str) {
        self.colored_log("ERROR", message);
    }

    // pub fn warning(&self, message: &str) {
    //     self.colored_log("WARNING", message);
    // }

    pub fn info(&self, message: &str) {
        self.colored_log("INFO", message);
    }

    pub fn debug(&self, message: &str) {
        self.colored_log("DEBUG", message);
    }
}
