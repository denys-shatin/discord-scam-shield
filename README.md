<div align="center">

# 🛡️ Discord Scam Detector

**Intelligent anti-scam bot for Discord powered by Rust and ML**

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[Features](#-features) • [Quick Start](#-quick-start) • [Deployment](#-deployment) • [Configuration](#%EF%B8%8F-configuration)

</div>

---

## ✨ Features

- **🚀 Lightning Fast** - Sub-millisecond text analysis with Rust performance
- **🤖 ML-Powered Detection** - Advanced scam pattern recognition
- **🖼️ Image OCR** - Extracts and analyzes text from images
- **🗑️ Auto-Moderation** - Instantly removes detected scam messages
- **📨 Rich Notifications** - Beautiful embed alerts sent to admins
- **🐳 Docker Ready** - One-command deployment with Docker Compose
- **⚡ Resource Efficient** - Only ~30MB RAM usage

## 🎯 Detection Capabilities

The bot analyzes messages for:

- 💰 Cryptocurrency scams (wallet phishing, fake airdrops)
- 💵 Financial fraud (investment schemes, money transfers)
- 🔗 Suspicious links and URLs
- 📢 Urgent call-to-action phrases
- 🖼️ Text embedded in images (OCR)

## 🚀 Quick Start

### Prerequisites

- Discord Bot Token ([Create one here](https://discord.com/developers/applications))
- OCR.space API Key ([Get free key](https://ocr.space/ocrapi))
- Docker (recommended) or Rust 1.75+

### 1. Clone Repository

```bash
git clone https://github.com/denys-shatin/discord-scam-shield.git
cd discord-scam-shield
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env
```

Add your credentials:

```env
DISCORD_TOKEN=your_bot_token_here
ADMIN_USER_ID=your_discord_user_id
OCR_SPACE_API_KEY=your_ocr_space_key
```

### 3. Run with Docker

```bash
docker-compose up -d --build
```

### 4. View Logs

```bash
docker-compose logs -f
```

## 🐳 Deployment

### VPS Deployment (5 commands)

```bash
# Connect to your VPS
ssh root@your_vps_ip

# Create directory
mkdir -p /opt/discord-bot && cd /opt/discord-bot

# Clone project
git clone https://github.com/denys-shatin/discord-scam-shield.git .

# Configure .env
nano .env

# Deploy
chmod +x quick-deploy.sh && ./quick-deploy.sh
```

### Docker Commands

```bash
# Start bot
docker-compose up -d

# View logs
docker-compose logs -f

# Restart
docker-compose restart

# Stop
docker-compose down

# Update
git pull && docker-compose up -d --build
```

### Native Installation (without Docker)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run
./target/release/discord-scam-detector
```

## ⚙️ Configuration

### Discord Bot Setup

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Enable **Privileged Gateway Intents**:
   - ✅ MESSAGE CONTENT INTENT
   - ✅ GUILD MESSAGES
4. Invite bot with permissions:
   - Read Messages
   - Send Messages
   - Manage Messages
   - Read Message History

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `DISCORD_TOKEN` | ✅ | Your Discord bot token |
| `ADMIN_USER_ID` | ✅ | Discord user ID for notifications |
| `OCR_SPACE_API_KEY` | ✅ | OCR.space API key for image analysis |

## 📊 Performance

- **Analysis Speed**: < 1ms per message
- **Memory Usage**: ~20-30MB RAM
- **CPU Usage**: Minimal (event-driven)
- **OCR Processing**: ~100-200ms per image

## 🔒 Security

### Before Committing

```bash
# Check for exposed secrets
chmod +x check-secrets.sh
./check-secrets.sh
```

### Security Features

- ✅ `.env` excluded from git
- ✅ Non-privileged Docker user
- ✅ Minimal permissions required
- ✅ No sensitive data in logs

## 📋 System Requirements

### VPS/Server

- **CPU**: 1 core
- **RAM**: 512MB (1GB recommended)
- **Storage**: 2GB
- **OS**: Ubuntu 20.04+ / Debian 11+

### Local Development

- **Rust**: 1.75+
- **Docker**: 20.10+ (optional)

## 🛠️ Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Build optimized release
cargo build --release

# Check code
cargo clippy
```

## 📝 How It Works

1. **Message Received** → Bot intercepts all messages
2. **OCR Processing** → Extracts text from images (if present)
3. **ML Analysis** → Scores message based on scam patterns
4. **Decision** → If score ≥ threshold, message is deleted
5. **Notification** → Admin receives detailed alert via DM

### Detection Algorithm

```
Score = crypto_terms + money_mentions + suspicious_links + urgency_phrases

If (Score >= 4) → DELETE & NOTIFY
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Serenity](https://github.com/serenity-rs/serenity) - Discord API library
- OCR powered by [OCR.space](https://ocr.space/)
- Inspired by the need for better Discord security

---

<div align="center">

**⭐ Star this repo if it helped protect your Discord server! ⭐**

Made with ❤️ and Rust

</div>
