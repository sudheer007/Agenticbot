#!/bin/bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y \
    ffmpeg \
    xvfb \
    x11-xserver-utils \
    pulseaudio \
    alsa-utils \
    build-essential \
    pkg-config \
    curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Create directories
sudo mkdir -p /recordings
sudo chmod 777 /recordings

# Create Xvfb start script
cat > start-xvfb.sh << EOL
#!/bin/bash
Xvfb :0 -screen 0 1920x1080x24 &
export DISPLAY=:0
pulseaudio --start
EOL

chmod +x start-xvfb.sh

# Create service file
sudo tee /etc/systemd/system/meeting-recorder.service << EOL
[Unit]
Description=Meeting Recorder Service
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/meeting-recorder
ExecStartPre=/opt/meeting-recorder/start-xvfb.sh
ExecStart=/opt/meeting-recorder/target/release/meeting-recorder
Restart=always
Environment=RUST_LOG=info
Environment=DISPLAY=:0

[Install]
WantedBy=multi-user.target
EOL