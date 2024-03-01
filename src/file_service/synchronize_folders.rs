use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::logger::Logger;

use super::copy_file_and_folders::copy_single_file_with_progress;

pub fn synchronize_folders(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // Create Destination Directory If it Doesn't Exist
    fs::create_dir_all(&dst)?;

    // Ensure source and destination are directories
    let src_path = src.as_ref();
    let dst_path = dst.as_ref();

    if !src_path.is_dir() || !dst_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source and destination must be directories",
        ));
    }

    for entry in fs::read_dir(&src_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();
        Logger.debug(&format!(
            "Synchronizing {} with {}...",
            path.to_str().unwrap(),
            dst_path.to_str().unwrap()
        ));

        match relative_path_without_prefix(src.as_ref(), path.as_path()) {
            Some(relative_path) => {
                let dst_path = dst_path.join(relative_path);

                if ty.is_dir() {
                    let result =
                        synchronize_folders(path.to_str().unwrap(), dst_path.to_str().unwrap());

                    match result {
                        Ok(_) => Logger.success(&format!(
                            "Synchronized {} with {} Successfully",
                            path.to_str().unwrap(),
                            dst_path.to_str().unwrap()
                        )),
                        Err(e) => {
                            Logger.error(&format!(
                                "Synchronizing Failed from {} with {} with error",
                                path.to_str().unwrap(),
                                dst_path.to_str().unwrap()
                            ));
                            println!("{}", e)
                        }
                    }
                } else {
                    if fs::metadata(&dst_path).is_ok() {
                        Logger.info("File already exists at the destination.");
                    } else {
                        let dst_parent = dst_path.parent().ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Invalid destination directory",
                            )
                        })?;
                        let result = copy_single_file_with_progress(&path, &dst_parent);
                        match result {
                            Ok(_) => Logger.success(&format!(
                                "Copyied from {} to {} Successfully",
                                path.to_str().unwrap(),
                                dst_path.to_str().unwrap()
                            )),
                            Err(e) => {
                                Logger.error("Synchronizing Failed with error");
                                println!("{}", e);
                            }
                        }
                    }
                }
            }
            None => {
                println!("Paths are not related")
            }
        }
    }

    // Logger.info("Deleting Files...");
    // for entry in fs::read_dir(&dst_path)? {
    //     let entry = entry?;
    //     let ty = entry.file_type()?;
    //     let path = entry.path();

    //     if (fs::metadata(src).is_ok()) {

    //     }
    // }
    // Delete files in destination that don't exist in source
    // for file in files_to_delete {
    // let relative_path = file.strip_prefix(dst_path).unwrap();
    // let dst_file = dst_path.join(relative_path);
    // Logger.info(&format!("Deleting: {}", dst_file.display()));
    // delete_single_file_with_progress(dst_file)?;
    // }

    Ok(())
}

fn relative_path_without_prefix(base: &Path, full_path: &Path) -> Option<PathBuf> {
    match full_path.strip_prefix(base) {
        Ok(relative_path) => Some(relative_path.to_path_buf()),
        Err(_) => None,
    }
}

// pub fn synchronize_folders(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
//     // CREATE DESTINATION DIRECTORY IF IT DOES NOT EXIST
//     fs::create_dir_all(&dst)?;

//     // Ensure source and destination are directories
//     let src_path = src.as_ref();
//     let dst_path = dst.as_ref();

//     if !src_path.is_dir() || !dst_path.is_dir() {
//         return Err(io::Error::new(
//             io::ErrorKind::InvalidInput,
//             "Source and destination must be directories",
//         ));
//     }

//     Logger.info("Checking Total Files...");
//     // Get lists of files in source and destination
//     let src_files = get_files_recursive(src_path)?;
//     let dst_files = get_files_recursive(dst_path)?;

//     Logger.info(&format!("Total files in source: {}", src_files.len()));
//     Logger.info(&format!("Total files in destination: {}", dst_files.len()));

//     // Calculate set differences for files to be copied and deleted
//     let files_to_copy: Vec<&Path> = src_files
//         .difference(&dst_files)
//         .map(|p| p.as_path())
//         .collect();

//     let files_to_delete: Vec<&Path> = dst_files
//         .difference(&src_files)
//         .map(|p| p.as_path())
//         .collect();

//     Logger.info(&format!("Files to Copy: {}", files_to_copy.len()));
//     Logger.info(&format!("Files to Delete: {}", files_to_delete.len()));

//     // Copy new files from source to destination
//     for file in files_to_copy {
//         let relative_path = file.strip_prefix(src_path).unwrap();
//         let dst_file = dst_path.join(relative_path);

//         // Check if the destination file already exists
//         if !dst_file.exists() {
//             // Ensure the parent directory exists in the destination
//             if let Some(parent) = dst_file.parent() {
//                 fs::create_dir_all(parent)?;
//             }

//             let dst_parent = dst_file.parent().ok_or_else(|| {
//                 std::io::Error::new(
//                     std::io::ErrorKind::InvalidInput,
//                     "Invalid destination directory",
//                 )
//             })?;
//             Logger.info(&format!("Copying: {}", file.display()));
//             copy_single_file_with_progress(file.to_str().unwrap(), dst_parent.to_str().unwrap())?;
//         } else {
//             Logger.info(&format!(
//                 "Skipping (File already exists): {}",
//                 file.display()
//             ));
//         }
//     }

//     Logger.info("Deleting Files...");
//     // Delete files in destination that don't exist in source
//     for file in files_to_delete {
//         let relative_path = file.strip_prefix(dst_path).unwrap();
//         let dst_file = dst_path.join(relative_path);
//         Logger.info(&format!("Deleting: {}", dst_file.display()));
//         delete_single_file_with_progress(dst_file)?;
//     }

//     Logger.info("Synchronization Completed.");
//     Ok(())
// }

// fn get_files_recursive(folder_path: impl AsRef<Path>) -> io::Result<HashSet<PathBuf>> {
//     let mut files = HashSet::new();

//     for entry in fs::read_dir(&folder_path)? {
//         let entry = entry?;
//         let ty = entry.file_type()?;
//         let path = entry.path();

//         if ty.is_dir() {
//             files.extend(get_files_recursive(&path)?);
//         } else {
//             files.insert(path);
//         }
//     }

//     Ok(files)
// }
