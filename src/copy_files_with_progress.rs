use std::fs;
use std::io::{self, Read, Write, Error, ErrorKind};
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

pub fn delete_folder_with_progress(folder_path: impl AsRef<Path>) -> io::Result<()> {
    // INITIALIZE LOGGER INSTANCE
    let logger = Logger;

    // CHECK IF THE FOLDER EXISTS
    if !folder_path.as_ref().exists() {
        return Err(Error::new(ErrorKind::NotFound, "Folder not found"));
    }

    // GET TOTAL SIZE OF THE FOLDER FOR PROGRESS BAR
    let total_size = calculate_folder_size(&folder_path)?;

    // INITIALIZE PROGRESS BAR
    let pb = ProgressBar::new(total_size);
    // pb.set_style(
    //     ProgressStyle::default_bar()
    //         .template("{spinner:.red} [{elapsed_precise}] [{wide_bar:.red/yellow}] {bytes}/{total_bytes} ({eta})")
    // )
    // .unwrap()
    // .progress_chars("#>-");
    pb.set_style(ProgressStyle::with_template("{spinner:.cyan} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));

    // DELETE FOLDER RECURSIVELY
    delete_folder_recursive_with_progress(&folder_path, &pb, &logger)?;

    // FINISH PROGRESS BAR WITH "DONE" MESSAGE
    pb.finish_with_message("done");

    Ok(())
}

fn delete_folder_recursive_with_progress(folder_path: impl AsRef<Path>, pb: &ProgressBar, logger: &Logger) -> io::Result<()> {
    // ITERATE THROUGH THE ENTRIES IN THE FOLDER
    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        // CHECK IF ENTRY IS A DIRECTORY
        if ty.is_dir() {
            // RECURSIVELY CALL THE FUNCTION FOR SUBDIRECTORIES
            delete_folder_recursive_with_progress(entry.path(), pb, logger)?;
        } else {
            // LOG INFORMATION ABOUT THE FILE BEING DELETED
            logger.info(&format!("Deleting File {}", entry.file_name().to_str().unwrap()));

            // GET FILE LENGTH FOR PROGRESS BAR
            let file_len = entry.metadata()?.len();

            // INCREMENT PROGRESS BAR
            pb.inc(file_len);
            
            // DELETE FILE
            fs::remove_file(entry.path())?;
        }
    }

    // DELETE THE EMPTY FOLDER
    fs::remove_dir(&folder_path)?;

    Ok(())
}

fn calculate_folder_size(folder_path: impl AsRef<Path>) -> io::Result<u64> {
    let mut total_size = 0;

    for entry in fs::read_dir(&folder_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            // RECURSIVELY CALCULATE SIZE OF SUBDIRECTORIES
            total_size += calculate_folder_size(entry.path())?;
        } else {
            // ADD FILE SIZE TO TOTAL SIZE
            total_size += entry.metadata()?.len();
        }
    }

    Ok(total_size)
}