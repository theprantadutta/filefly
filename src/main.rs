use std::env;
use std::time::Instant;

use copy_files_with_progress::delete_folder_with_progress;

use crate::copy_files_with_progress::copy_files_with_progress;
use crate::logger::Logger;

mod copy_files_with_progress;
mod logger;


fn main() {
    // Record the start time
    let start_time = Instant::now();
    Logger.info("Starting The Copy Script...");

    if let Ok(current_dir) = env::current_dir() {
        // Assuming your project root is the directory containing the Cargo.toml file
        let copy_from = current_dir.join("temp");
        
        // let copy_to = current_dir.join("output");
        let copy_to = current_dir.join(r"C:\Users\prant\Downloads\Compressed");
        Logger.debug(&format!("Copying All Files From {} To {}", copy_from.to_str().unwrap(), copy_to.to_str().unwrap()));
        // let result = copy_files_with_progress(&copy_from, &copy_to);
        let result = delete_folder_with_progress(copy_to);

        match result {
            Ok(_) => Logger.success("Copying Successful"),
            Err(_) => Logger.error("Copying Failed")
        }

        // Calculate and print the elapsed time
        let elapsed_time = start_time.elapsed();
        Logger.info(&format!("Time taken: {:?}", elapsed_time));
    } else {
        eprintln!("Failed to get the current working directory.");
    }
}