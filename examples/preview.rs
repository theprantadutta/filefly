use indicatif::{ProgressBar, ProgressDrawTarget};
use std::thread;
use std::time::Duration;

#[path = "../src/progress_style.rs"]
mod progress_style;

fn demo(prefix: String, style: indicatif::ProgressStyle) {
    let total: u64 = 100;
    let pb = ProgressBar::with_draw_target(Some(total), ProgressDrawTarget::stderr_with_hz(30));
    pb.set_style(style);
    pb.set_prefix(prefix);
    pb.enable_steady_tick(Duration::from_millis(90));
    for i in 0..=total {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(15));
    }
    pb.finish();
}

fn main() {
    demo(progress_style::copy_prefix("short.mp4"), progress_style::copy_style());
    demo(
        progress_style::copy_prefix("Channel Zero - S01E01.mkv"),
        progress_style::copy_style(),
    );
    demo(
        progress_style::copy_prefix("DARK.S01E03.Past.and.Present.720p.NF.WEB-DL.MkvCage.mkv"),
        progress_style::copy_style(),
    );
    demo(
        progress_style::delete_prefix("old_cache_dir"),
        progress_style::delete_style(),
    );
}
