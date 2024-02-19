// use std::sync::Arc;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::time::Duration;
// use crate::logger::Logger;


// pub fn wait_for_ctrl_c() {
//     Logger.info("Press Ctrl+C to Exit...");
//     let running = Arc::new(AtomicBool::new(true));

//     // Setup Ctrl+C handler
//     let running_clone = running.clone();
//     ctrlc::set_handler(move || {
//         running_clone.store(false, Ordering::SeqCst);
//     }).expect("Error setting Ctrl+C handler");

//     // Loop until Ctrl+C is received
//     while running.load(Ordering::SeqCst) {
//         std::thread::sleep(Duration::from_secs(1));
//     }
// }