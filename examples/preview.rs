use indicatif::{ProgressBar, ProgressDrawTarget};
use std::thread;
use std::time::Duration;

#[path = "../src/progress_style.rs"]
mod progress_style;

fn demo(name: &str, style: indicatif::ProgressStyle, glyph: &str) {
    let total: u64 = 100;
    let pb = ProgressBar::with_draw_target(Some(total), ProgressDrawTarget::stderr_with_hz(30));
    pb.set_style(style);
    pb.set_prefix(format!("{} {}", glyph, name));
    pb.enable_steady_tick(Duration::from_millis(90));
    for i in 0..=total {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(15));
    }
    pb.finish();
}

fn main() {
    demo("big_video_file.mp4", progress_style::copy_style(), "\u{21AA}");
    demo("old_cache_dir", progress_style::delete_style(), "\u{2716}");
}
