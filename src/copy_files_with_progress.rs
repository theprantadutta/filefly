use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use crate::logger::Logger;

const BUFFER_SIZE: usize = 8192;

pub fn copy_files_with_progress(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // INITIALIZE LOGGER INSTANCE
    let logger = Logger;

    // CREATE DESTINATION DIRECTORY IF IT DOES NOT EXIST
    fs::create_dir_all(&dst)?;

    // ITERATE THROUGH THE ENTRIES IN THE SOURCE DIRECTORY
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        // CHECK IF ENTRY IS A DIRECTORY
        if ty.is_dir() {
            // RECURSIVELY CALL THE FUNCTION FOR SUBDIRECTORIES
            copy_files_with_progress(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            // LOG INFORMATION ABOUT THE FILE BEING COPIED
            logger.info(&format!("Copying Files From {}", entry.file_name().to_str().unwrap()));

            // GET FILE LENGTH FOR PROGRESS BAR
            let file_len = entry.metadata()?.len();

            // INITIALIZE PROGRESS BAR
            let pb = ProgressBar::new(file_len);
            // pb.set_style(ProgressStyle::default_bar()
            //     .template("[{spinner:.cyan} {elapsed_precise}] {bar:40.cyan/blue} {percent}% ({eta})")
            //     .unwrap().progress_chars("##-"));
            pb.set_style(ProgressStyle::with_template("{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));

            // OPEN SOURCE FILE FOR READING
            let mut src_file = fs::File::open(&entry.path())?;

            // CREATE DESTINATION FILE FOR WRITING
            let mut dst_file = fs::File::create(dst.as_ref().join(entry.file_name()))?;

            // INITIALIZE BUFFER FOR FILE COPY
            let mut buffer = [0u8; BUFFER_SIZE];

            // MAIN LOOP FOR FILE COPY
            loop {
                match src_file.read(&mut buffer) {
                    Ok(0) => break, // REACHED THE END OF THE FILE
                    Ok(bytes_read) => {
                        // WRITE BUFFER TO DESTINATION FILE
                        dst_file.write_all(&buffer[..bytes_read])?;

                        // INCREMENT PROGRESS BAR
                        pb.inc(bytes_read as u64);
                    }
                    Err(err) => {
                        // HANDLE ERROR DURING FILE READ
                        eprintln!("Error reading file: {}", err);
                        pb.finish_with_message("error");
                        return Err(err);
                    }
                }
            }

            // FINISH PROGRESS BAR WITH "DONE" MESSAGE
            pb.finish_with_message("done");
        }
    }

    // RETURN OK RESULT IF EVERYTHING SUCCEEDS
    Ok(())
}