# Chequer

**Steam Remote Play Diagnostic Tool**

A comprehensive diagnostic tool for troubleshooting Steam Remote Play streaming issues between gaming PCs and Steam Deck clients.

## Overview

Chequer provides:
- Network latency and jitter testing
- Bandwidth measurement
- Video codec support detection
- Audio system verification
- Traffic light reporting (🟢 🟡 🔴)
- Actionable recommendations

## Architecture

- **Agent**: Diagnostic tool that runs on both host (gaming PC) and client (Steam Deck)
- **Test Game**: Bevy-based test application for generating controlled streaming workloads
- **Common**: Shared protocol and type definitions
- **Report**: Analysis and reporting engine

## Quick Start

### Build

```bash
cargo build --release
```

### Run Host (Gaming PC)

```bash
./target/release/chequer host --listen 0.0.0.0:7777
```

### Run Client (Steam Deck)

```bash
./target/release/chequer client --connect 192.168.1.100:7777
```

### Run Test Game

```bash
./target/release/chequer-test-game
```

## Development Status

🚧 **Vertical Slice in Progress**

Current focus: Basic network latency testing between client and host.

## Project Structure

```
chequer/
├── crates/
│   ├── agent/          # CLI diagnostic tool
│   ├── test-game/      # Bevy test application
│   ├── common/         # Shared types and protocol
│   └── report/         # Report generation
├── Cargo.toml          # Workspace configuration
└── README.md
```

## License

MIT OR Apache-2.0
