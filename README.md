After your server is ready

ssh username@server_ip

# Update package list
sudo apt-get update

# Install required packages
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libx11-dev \
    libxext-dev \
    libxft-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxrender-dev \
    libxfixes-dev \
    libxtst-dev \
    libxrandr-dev \
    curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure your shell
source $HOME/.cargo/env

# Set environment variables (if needed)
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include/openssl
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig

# Clone your repository
cd /opt
sudo git clone --branch 2 git@github.com:sudheer007/Agenticbot.git meeting-recorder

# Navigate to your project directory
cd meeting-recorder

# Build your project
cargo build --release

# Create the deployment script
sudo nano deploy.sh

# (Paste the deployment script content here)

# Make the deployment script executable
sudo chmod +x deploy.sh

# Run the deployment script
sudo bash deploy.sh

# Check the service status
sudo systemctl status meeting-recorder
