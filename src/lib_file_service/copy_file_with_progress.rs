extern crate fs_extra;

use std::io;
use fs_extra::{copy_items_with_progress, dir, TransitProcess};

pub fn copy_file_with_progress() -> io::Result<()> {
    let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
    let handle = |process_info: TransitProcess| {
        println!("{}", process_info.total_bytes);
        fs_extra::dir::TransitProcessResult::ContinueOrAbort
    };
    // copy dir1 and file1.txt to target/dir1 and target/file1.txt
    let mut from_paths = Vec::new();
    from_paths.push("temp");
    // from_paths.push("source/file.txt");
    copy_items_with_progress(&from_paths, "output", &options, handle)?;
    return Ok(());
}