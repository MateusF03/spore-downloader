use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{wide_bar}] {human_pos}/{human_len} ({percent}%) {per_sec} ETA: {eta} {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("██░"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}
