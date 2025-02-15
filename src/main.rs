use anyhow::{Result, Context};
use chrono::Local;
use log::info;
use std::process::Command;
use std::time::Duration;
use tokio::time;
use std::process::Command as ProcessCommand;
use rdev::{simulate, EventType, Key};
use std::thread;
use std::fs::File;

struct MeetingRecorder {
    meeting_url: String,
    output_dir: String,
}

impl MeetingRecorder {
    fn new(meeting_url: String, output_dir: String) -> Self {
        Self {
            meeting_url,
            output_dir,
        }
    }

    async fn open_browser(&self) -> Result<()> {
        info!("Opening browser with URL: {}", self.meeting_url);
        
        ProcessCommand::new("chromium-browser")
            .env("DISPLAY", ":0")
            .args(&[
                "--no-sandbox",
                "--disable-gpu",
                "--disable-dev-shm-usage",
                "--use-fake-ui-for-media-stream",
                "--use-fake-device-for-media-stream",
                "--autoplay-policy=no-user-gesture-required",
                &self.meeting_url
            ])
            .spawn()
            .context("Failed to open browser")?;

        time::sleep(Duration::from_secs(15)).await;
        Ok(())
    }

    async fn join_meeting(&self) -> Result<()> {
        self.open_browser().await?;
        time::sleep(Duration::from_secs(20)).await;

        // Clear input box
        let _ = simulate(&EventType::KeyPress(Key::ControlLeft));
        let _ = simulate(&EventType::KeyPress(Key::KeyA));
        let _ = simulate(&EventType::KeyRelease(Key::KeyA));
        let _ = simulate(&EventType::KeyRelease(Key::ControlLeft));
        time::sleep(Duration::from_millis(200)).await;

        let _ = simulate(&EventType::KeyPress(Key::Backspace));
        let _ = simulate(&EventType::KeyRelease(Key::Backspace));
        time::sleep(Duration::from_millis(200)).await;

        // Type name
        for c in "Graycommit".chars() {
            let key = match c {
                'G' => Key::KeyG,
                'r' => Key::KeyR,
                'a' => Key::KeyA,
                'y' => Key::KeyY,
                'c' => Key::KeyC,
                'o' => Key::KeyO,
                'm' => Key::KeyM,
                'i' => Key::KeyI,
                't' => Key::KeyT,
                _ => continue,
            };
            let _ = simulate(&EventType::KeyPress(key));
            thread::sleep(Duration::from_millis(200));
            let _ = simulate(&EventType::KeyRelease(key));
            thread::sleep(Duration::from_millis(200));
        }

        time::sleep(Duration::from_secs(2)).await;
        
        let _ = simulate(&EventType::KeyPress(Key::Tab));
        thread::sleep(Duration::from_millis(200));
        let _ = simulate(&EventType::KeyRelease(Key::Tab));
        time::sleep(Duration::from_secs(2)).await;

        let _ = simulate(&EventType::KeyPress(Key::Return));
        thread::sleep(Duration::from_millis(200));
        let _ = simulate(&EventType::KeyRelease(Key::Return));
        time::sleep(Duration::from_secs(15)).await;

        Ok(())
    }

    async fn start_recording(&self) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let output_file = format!("{}/meeting_{}.mp4", self.output_dir, timestamp);
        let log_file = format!("{}/ffmpeg_log_{}.txt", self.output_dir, timestamp);

        let ffmpeg_args = vec![
    "-f", "x11grab",
    "-framerate", "5",
    "-video_size", "800x600",
    "-i", ":0.0",
    "-f", "pulse",
    "-i", "v_speaker",  // Changed from "default" to "v_speaker"
    "-c:v", "libx264",
    "-preset", "ultrafast",
    "-crf", "40",
    "-c:a", "aac",
    "-ac", "1",
    "-ar", "16000",
    "-b:a", "24k",
    "-threads", "2",
    "-y",
    &output_file
];

        let mut child = Command::new("ffmpeg")
            .args(&ffmpeg_args)
            .stdout(File::create(&log_file)?)
            .stderr(File::create(&log_file)?)
            .spawn()
            .context("Failed to start FFmpeg")?;

        tokio::spawn(async move {
            let _ = child.wait();
        });

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let recorder = MeetingRecorder::new(
        "https://logsimpl.com/yo".to_string(),
        "/recordings".to_string(),
    );

    recorder.join_meeting().await?;
    recorder.start_recording().await?;

    loop {
        time::sleep(Duration::from_secs(1)).await;
    }
}