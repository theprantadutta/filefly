use clap::Parser;

use handlers::{handle_copy_command, handle_delete_command};
use utils::wait_for_ctrl_c;
use logger::Logger;

mod logger;
mod handlers;
mod utils;
mod file_service;

#[derive(Parser, Debug)]
#[clap(name = "command")]
pub enum Operation {
    #[clap(short_flag = 'c', alias = "cp", name = "copy", about = "Copy files from source to destination")]
    Copy(CopyCommand),

    #[clap(short_flag = 'd', alias = "del", name = "delete", about = "Delete a folder")]
    Delete(DeleteCommand),
}

#[derive(Parser, Debug)]
pub struct CopyCommand {
    /// Source directory
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    source: String,

    /// Destination directory
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    destination: String,
}

#[derive(Parser, Debug)]
pub struct DeleteCommand {
    /// Folder to delete
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    folder: String,
}

fn main() {
    // Record the start time
    let start_time = std::time::Instant::now();

    // Parse command-line arguments
    let args = Operation::parse();

    match args {
        Operation::Copy(command) => handle_copy_command(command),
        Operation::Delete(command) => handle_delete_command(command),
    }

    // Calculate and print the elapsed time
    let elapsed_time = start_time.elapsed();
    Logger.info(&format!(
        "Time taken: {:.2} seconds ({:.2} milliseconds)",
        elapsed_time.as_secs_f64(),
        elapsed_time.as_millis() as f64
    ));

    // Wait for Ctrl+C to exit
    wait_for_ctrl_c();
}