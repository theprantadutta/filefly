use colored::*;
use chrono::{Local, DateTime};

// DEFINE A LOGGER STRUCT
pub struct Logger;

impl Logger {
    // FUNCTION TO GET THE CURRENT TIMESTAMP
    fn get_timestamp() -> DateTime<Local> {
        Local::now()
    }

    // FUNCTION TO FORMAT THE TIMESTAMP AS A STRING
    fn format_timestamp(timestamp: DateTime<Local>) -> String {
        // FORMAT THE TIMESTAMP WITH YEAR-MONTH-DAY HOUR:MINUTE:SECOND AM/PM
        timestamp.format("%Y-%m-%d %I:%M:%S %p").to_string()
    }

    // FUNCTION TO APPLY COLOR TO THE LOG MESSAGE BASED ON LOG LEVEL
    fn colored_log(&self, level: &str, message: &str) {
        // GET THE CURRENT TIMESTAMP
        let timestamp = Self::get_timestamp();
        // FORMAT THE TIMESTAMP AS A STRING
        let formatted_timestamp = Self::format_timestamp(timestamp);

        // CREATE A FORMATTED LOG MESSAGE INCLUDING TIMESTAMP, LEVEL, AND THE ACTUAL MESSAGE
        let formatted_message = format!("{} [{}]: {}", formatted_timestamp, level, message);

        // PRINT THE LOG MESSAGE WITH COLOR BASED ON THE LOG LEVEL
        match level {
            "SUCCESS" => println!("{}", formatted_message.bright_green()), // APPLY BRIGHT GREEN COLOR TO SUCCESS LOGS
            "ERROR" => println!("{}", formatted_message.bright_red()), // APPLY BRIGHT RED COLOR TO ERROR LOGS
            "WARNING" => println!("{}", formatted_message.bright_yellow()), // APPLY BRIGHT YELLOW COLOR TO WARNING LOGS
            "INFO" => println!("{}", formatted_message.bright_cyan()), // APPLY BRIGHT CYAN COLOR TO INFO LOGS
            "DEBUG" => println!("{}", formatted_message.bright_blue()), // APPLY BRIGHT BLUE COLOR TO DEBUG LOGS
            _ => println!("{}", formatted_message), // DEFAULT COLOR FOR UNRECOGNIZED LOG LEVELS
        }
    }

    // LOG FUNCTION THAT CALLS colored_log WITH A LOG LEVEL OF "LOG"
    // pub fn log(&self, message: &str) {
    //     self.colored_log("LOG", message);
    // }

    // CONVENIENCE FUNCTIONS FOR DIFFERENT LOG LEVELS THAT CALL colored_log WITH THEIR RESPECTIVE LOG LEVEL
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
