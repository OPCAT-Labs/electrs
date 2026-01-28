# Installation Guide

This guide covers installing electrs from pre-built binaries.

## System Requirements

- **Operating System**: Linux (x86_64)
- **Memory**: 4GB RAM minimum (8GB+ recommended)
- **Disk Space**: 50GB+ for Bitcoin mainnet index
- **Bitcoin Core**: Fully synced Bitcoin node with txindex enabled

## Quick Start

### 1. Download Binary

Choose the appropriate binary for your system:

**MUSL (Recommended - Static Binary)**
```bash
VERSION=3.3.0
wget https://github.com/OPCAT-Labs/electrs/releases/download/v${VERSION}/electrs-v${VERSION}-linux-x86_64-musl.tar.gz
```

**GNU (Dynamic Binary - Requires librocksdb)**
```bash
VERSION=3.3.0
wget https://github.com/OPCAT-Labs/electrs/releases/download/v${VERSION}/electrs-v${VERSION}-linux-x86_64-gnu.tar.gz
```

### 2. Verify Download

Download checksums and signature:
```bash
wget https://github.com/OPCAT-Labs/electrs/releases/download/v${VERSION}/SHA256SUMS
wget https://github.com/OPCAT-Labs/electrs/releases/download/v${VERSION}/SHA256SUMS.asc
```

Verify checksum:
```bash
sha256sum -c SHA256SUMS --ignore-missing
```

(Optional) Verify GPG signature:
```bash
# Import GPG key (first time only)
curl -sL https://raw.githubusercontent.com/OPCAT-Labs/electrs/main/KEYS | gpg --import

# Verify signature
gpg --verify SHA256SUMS.asc SHA256SUMS
```

### 3. Extract and Install

```bash
# Extract
tar xzf electrs-v${VERSION}-linux-x86_64-musl.tar.gz
cd electrs-v${VERSION}-linux-x86_64-musl

# Install binary
sudo cp bin/electrs /usr/local/bin/
sudo chmod +x /usr/local/bin/electrs

# Verify installation
electrs --version
```

## Configuration

### 4. Create User and Directories

```bash
# Create dedicated user
sudo useradd -r -s /bin/false electrs

# Create data directory
sudo mkdir -p /var/lib/electrs/db
sudo chown -R electrs:electrs /var/lib/electrs
```

### 5. Configure Bitcoin Core

Ensure Bitcoin Core is configured with `txindex=1` in `bitcoin.conf`:

```ini
# bitcoin.conf
txindex=1
server=1
rpcuser=your_rpc_user
rpcpassword=your_rpc_password
```

Restart Bitcoin Core after making changes.

### 6. Run Electrs

Basic usage:
```bash
electrs --db-dir /var/lib/electrs/db --daemon-dir /var/lib/bitcoind
```

Common options:
- `--network <network>`: Network type (mainnet, testnet, regtest)
- `--electrum-rpc-addr <addr>`: Electrum server address (default: 127.0.0.1:50001)
- `--daemon-rpc-addr <addr>`: Bitcoin RPC address
- `--monitoring-addr <addr>`: Prometheus monitoring address

## Running as a Service

### systemd Setup

Install the systemd service file:

```bash
sudo cp contrib/electrs.service /etc/systemd/system/
sudo systemctl daemon-reload
```

Edit the service file to match your configuration:
```bash
sudo nano /etc/systemd/system/electrs.service
```

Start and enable the service:
```bash
sudo systemctl start electrs
sudo systemctl enable electrs
sudo systemctl status electrs
```

View logs:
```bash
sudo journalctl -u electrs -f
```

## Platform-Specific Notes

### MUSL vs GNU Binaries

**MUSL (Recommended)**
- Statically linked, no dependencies required
- Works on any Linux distribution
- Slightly larger binary size
- Best for portability

**GNU**
- Dynamically linked to system libraries
- Requires librocksdb installed on the system
- Smaller binary size
- Install dependencies: `sudo apt-get install librocksdb-dev`

## Troubleshooting

### Connection Issues

If electrs cannot connect to Bitcoin Core:
- Verify Bitcoin Core is running and synced
- Check RPC credentials in bitcoin.conf
- Ensure `txindex=1` is enabled

### Permission Errors

If you encounter permission errors:
```bash
sudo chown -R electrs:electrs /var/lib/electrs
sudo chmod 755 /var/lib/electrs
```

### Memory Issues

Electrs requires significant memory during initial sync:
- Minimum 4GB RAM
- 8GB+ recommended for better performance
- Consider adding swap space if needed

## Alternative: Running with PM2

PM2 is a popular process manager for Node.js applications, but it also works well for managing any long-running process.

### Install PM2

```bash
# Install Node.js and npm if not already installed
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install PM2 globally
sudo npm install -g pm2
```

### Create PM2 Ecosystem File

Create a file `ecosystem.config.js`:

```javascript
module.exports = {
  apps: [{
    name: 'electrs',
    script: '/usr/local/bin/electrs',
    args: '--db-dir /var/lib/electrs/db --daemon-dir /var/lib/bitcoind',
    cwd: '/var/lib/electrs',
    user: 'electrs',
    env: {
      RUST_BACKTRACE: '1'
    },
    max_memory_restart: '4G',
    error_file: '/var/log/electrs/error.log',
    out_file: '/var/log/electrs/out.log',
    log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
    merge_logs: true,
    autorestart: true,
    max_restarts: 10,
    min_uptime: '10s'
  }]
};
```

### Start with PM2

```bash
# Create log directory
sudo mkdir -p /var/log/electrs
sudo chown electrs:electrs /var/log/electrs

# Start electrs with PM2
pm2 start ecosystem.config.js

# Save PM2 configuration
pm2 save

# Setup PM2 to start on boot
pm2 startup systemd -u electrs --hp /home/electrs
```

### PM2 Management Commands

```bash
# View status
pm2 status

# View logs
pm2 logs electrs

# Restart
pm2 restart electrs

# Stop
pm2 stop electrs

# Monitor
pm2 monit
```

## Upgrading

To upgrade to a new version:

1. Stop the running service
2. Download and verify the new binary
3. Replace the old binary
4. Restart the service

```bash
# Stop service
sudo systemctl stop electrs  # or: pm2 stop electrs

# Download new version
VERSION=3.3.1
wget https://github.com/OPCAT-Labs/electrs/releases/download/v${VERSION}/electrs-v${VERSION}-linux-x86_64-musl.tar.gz

# Verify and extract
sha256sum -c SHA256SUMS --ignore-missing
tar xzf electrs-v${VERSION}-linux-x86_64-musl.tar.gz

# Replace binary
sudo cp electrs-v${VERSION}-linux-x86_64-musl/bin/electrs /usr/local/bin/

# Restart service
sudo systemctl start electrs  # or: pm2 restart electrs
```

## Additional Resources

- [GitHub Repository](https://github.com/OPCAT-Labs/electrs)
- [Security Policy](../SECURITY.md)
- [Main README](../README.md)

## Support

For issues and questions:
- Open an issue on GitHub
- Check existing documentation
- Review logs for error messages
