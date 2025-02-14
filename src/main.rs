use anyhow::{Result, Context};
use chrono::Local;

use log::{info};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;
use std::process::Command as ProcessCommand;
use rdev::{simulate, EventType, Key};
use std::thread;

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
        
        ProcessCommand::new("open")
            .arg(&self.meeting_url)
            .spawn()
            .context("Failed to open browser")?;

        // Wait longer for page to fully load
        time::sleep(Duration::from_secs(10)).await;
        
        Ok(())
    }

    async fn join_meeting(&self) -> Result<()> {
        self.open_browser().await?;
        
        info!("Waiting for page load");
        time::sleep(Duration::from_secs(20)).await;
        
        // Type Graycommit character by character
        info!("Typing Graycommit");
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
            thread::sleep(Duration::from_millis(100));
            let _ = simulate(&EventType::KeyRelease(key));
            thread::sleep(Duration::from_millis(50));
        }
        
        info!("Waiting after typing name");
        time::sleep(Duration::from_secs(2)).await;
        
        // Press Tab
        info!("Pressing Tab");
        let _ = simulate(&EventType::KeyPress(Key::Tab));
        thread::sleep(Duration::from_millis(100));
        let _ = simulate(&EventType::KeyRelease(Key::Tab));
        time::sleep(Duration::from_secs(2)).await;
        
        // Press Enter
        info!("Pressing Enter to join");
        let _ = simulate(&EventType::KeyPress(Key::Return));
        thread::sleep(Duration::from_millis(100));
        let _ = simulate(&EventType::KeyRelease(Key::Return));
        
        // Wait for meeting join
        info!("Waiting for meeting to join");
        time::sleep(Duration::from_secs(10)).await;
        
        Ok(())
    }

    async fn start_recording(&self) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let output_file = format!("{}/meeting_{}.mp4", self.output_dir, timestamp);

        // FFmpeg settings for macOS
        let ffmpeg_args = vec![
            "-f", "avfoundation",
            "-i", "1:0",              // "1" is screen, "0" is audio device
            "-framerate", "10",       // Low framerate for efficiency
            "-video_size", "800x600", // Small resolution
            "-c:v", "libx264",
            "-preset", "superfast",
            "-crf", "35",            // High compression
            "-c:a", "aac",
            "-ac", "1",              // Mono audio
            "-ar", "22050",          // Low sample rate
            "-b:a", "32k",           // Low audio bitrate
            "-y",
            &output_file
        ];

        let mut child = Command::new("ffmpeg")
            .args(&ffmpeg_args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start FFmpeg")?;

        info!("Started recording to: {}", output_file);

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
        "recordings".to_string(),
    );

    recorder.join_meeting().await?;
    recorder.start_recording().await?;

    loop {
        time::sleep(Duration::from_secs(1)).await;
    }
}