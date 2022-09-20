use std::{cmp::min, thread, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut downloaded = 0;
    let total_size = 31231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} \
             ({bytes_per_sec}, {eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
