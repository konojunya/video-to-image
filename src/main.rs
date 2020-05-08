use std::process::{Command, Stdio};

static SAMPLE_MP4: &str = "sample.mp4";

fn main() {
    let mut ffmpeg = Command::new("/bin/sh")
        .args(&[
            "-c",
            &format!("ffmpeg -i {i} -r 30 {o}%03d.png", i = SAMPLE_MP4, o = "mv"),
        ])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    ffmpeg.wait().unwrap();
}
