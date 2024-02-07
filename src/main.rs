use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::time::Duration;
use std::io::{self, Write};

use copy_files_with_progress::delete_folder_with_progress;

// use crate::copy_files_with_progress::copy_files_with_progress;
use crate::logger::Logger;

mod copy_files_with_progress;
mod logger;


fn main() {
    // Record the start time
    let start_time = Instant::now();
    Logger.info("Starting The Copy Script...");

    if let Ok(_current_dir) = env::current_dir() {
        // Assuming your project root is the directory containing the Cargo.toml file
        // let copy_from = current_dir.join("temp");
        
        // let copy_to = current_dir.join("output");
        // let copy_to = current_dir.join(r"C:\Users\prant\Downloads\Compressed");

        // Get the source directory from the user
        let copy_from = get_user_input("Enter the source directory: ");

        // Get the destination directory from the user
        // let copy_to = get_user_input("Enter the destination directory: ");
        
        // Logger.debug(&format!("Copying All Files From {} To {}", copy_from.as_str(), copy_to.as_str()));
        
        // let result = copy_files_with_progress(&copy_from, &copy_to);
        let result = delete_folder_with_progress(&copy_from);

        match result {
            Ok(_) => Logger.success("Copying Successful"),
            Err(e) => {
                 Logger.error("Copying Failed with error");
                 println!("{}", e);
            }
        }

        // Calculate and print the elapsed time
        let elapsed_time = start_time.elapsed();
        Logger.info(&format!("Time taken: {:.2} seconds ({:.2} milliseconds)", elapsed_time.as_secs_f64(), elapsed_time.as_millis() as f64));
    } else {
        eprintln!("Failed to get the current working directory.");
    }

    // Wait for Ctrl+C to exit
    wait_for_ctrl_c();
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn wait_for_ctrl_c() {
    Logger.info("Press Ctrl+C to Exit...");
    let running = Arc::new(AtomicBool::new(true));

    // Setup Ctrl+C handler
    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    // Loop until Ctrl+C is received
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(1));
    }
}