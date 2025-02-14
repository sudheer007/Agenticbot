#!/bin/bash
# Install minimal dependencies
sudo apt-get update
sudo apt-get install -y \
    ffmpeg \
    xvfb \
    pulseaudio \
    chromium-browser

# Create directories
sudo mkdir -p /opt/meeting-recorder
sudo mkdir -p /recordings
sudo chmod 777 /recordings

# Setup virtual display
cat > /opt/meeting-recorder/start-xvfb.sh << EOL
#!/bin/bash
Xvfb :0 -screen 0 800x600x16 &
export DISPLAY=:0
pulseaudio --start --exit-idle-time=-1
EOL

chmod +x /opt/meeting-recorder/start-xvfb.sh

# Setup audio
cat > /etc/pulse/default.pa << EOL
load-module module-native-protocol-unix
load-module module-virtual-sink sink_name=v_speaker
set-default-sink v_speaker
EOL

# Create service with resource limits
cat > /etc/systemd/system/meeting-recorder.service << EOL
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
Nice=10
CPUQuota=5%
MemoryLimit=50M

[Install]
WantedBy=multi-user.target
EOL

# Enable and start service
systemctl daemon-reload
systemctl enable meeting-recorder
systemctl start meeting-recorder