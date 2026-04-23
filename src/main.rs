use clap::Parser;
use filefly_args::FileFlyArgs;
use handlers::{
    handle_copy_command::handle_copy_command, handle_delete_command::handle_delete_command,
    handle_replace_command::handle_replace_command,
    handle_synchronize_command::handle_synchronize_command,
    handle_upgrade_command::handle_upgrade_command,
};
use logger::{LogLevel, Logger};

mod file_service;
mod filefly_args;
mod handlers;
mod logger;
mod progress_style;

fn main() {
    let args = FileFlyArgs::parse();

    let (no_log, log_level) = match &args {
        FileFlyArgs::Copy(c) => (c.no_log, c.log_level),
        FileFlyArgs::Delete(c) => (c.no_log, c.log_level),
        FileFlyArgs::Replace(c) => (c.no_log, c.log_level),
        FileFlyArgs::Synchronize(c) => (c.no_log, c.log_level),
        FileFlyArgs::Upgrade(_) => (false, LogLevel::Info),
    };

    let logger = Logger::new(Some(log_level), Some(no_log));

    let start_time = std::time::Instant::now();

    match args {
        FileFlyArgs::Copy(command) => handle_copy_command(command),
        FileFlyArgs::Delete(command) => handle_delete_command(command),
        FileFlyArgs::Replace(command) => handle_replace_command(command),
        FileFlyArgs::Synchronize(command) => handle_synchronize_command(command),
        FileFlyArgs::Upgrade(command) => {
            if let Err(e) = handle_upgrade_command(command) {
                logger.error(&format!("Upgrade failed: {}", e));
            }
        }
    }

    let elapsed_time = start_time.elapsed();
    logger.info(&format!(
        "Time taken: {:.2} seconds ({:.2} milliseconds)",
        elapsed_time.as_secs_f64(),
        elapsed_time.as_millis() as f64
    ));
}
