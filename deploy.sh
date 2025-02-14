#!/bin/bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y \
    ffmpeg \
    xvfb \
    pulseaudio \
    chromium-browser

# Create minimal Xvfb config
cat > start-xvfb.sh << EOL
#!/bin/bash
Xvfb :0 -screen 0 800x600x16 &
export DISPLAY=:0
pulseaudio --start --exit-idle-time=-1
EOL

chmod +x start-xvfb.sh

# Minimal PulseAudio config
sudo mkdir -p /etc/pulse
cat > /etc/pulse/default.pa << EOL
load-module module-native-protocol-unix
load-module module-virtual-sink sink_name=v_speaker
set-default-sink v_speaker
EOL

# Create directories
sudo mkdir -p /recordings
sudo chmod 777 /recordings

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

# Restart PulseAudio
systemctl --user restart pulseaudio