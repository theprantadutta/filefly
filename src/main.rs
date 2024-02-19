use clap::Parser;
use filefly_args::FileFlyArgs;
use handlers::{handle_copy_command::handle_copy_command, handle_delete_command::handle_delete_command};
// use utils::wait_for_ctrl_c;
use logger::Logger;

mod logger;
mod handlers;
mod utils;
mod file_service;
mod filefly_args;
mod lib_file_service;

fn main() {
    // Record the start time
    let start_time = std::time::Instant::now();

    // Parse command-line arguments
    let args = FileFlyArgs::parse();

    match args {
        FileFlyArgs::Copy(command) => handle_copy_command(command),
        FileFlyArgs::Delete(command) => handle_delete_command(command),
    }

    // Calculate and print the elapsed time
    let elapsed_time = start_time.elapsed();
    Logger.info(&format!(
        "Time taken: {:.2} seconds ({:.2} milliseconds)",
        elapsed_time.as_secs_f64(),
        elapsed_time.as_millis() as f64
    ));

    // Wait for Ctrl+C to exit
    // wait_for_ctrl_c();
}