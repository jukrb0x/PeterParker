# 🕸️ PeterParker - Network Scanner

Your friendly neighborhood network scanner. Discover devices on your local network with detailed information including MAC addresses, vendors, open ports, and more.

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- 🔍 **Network Discovery**: Scan your local network to find all connected devices
- 📡 **Multiple Scan Methods**: ARP, ICMP ping, and TCP port scanning
- 🏷️ **Device Identification**: MAC address detection with vendor lookup (44,000+ vendors)
- 🖥️ **Cross-Platform**: Works on macOS, Windows, and Linux
- ⚡ **Real-time Updates**: See scan progress and results in real-time
- 🎯 **Port Scanning**: Detect open ports and services
- 📊 **Device Classification**: Automatically identify device types (router, laptop, mobile, etc.)

## Quick Start

### Prerequisites

- **Node.js** 18+ and **pnpm** 9+
- **Rust** 1.75+ (for Tauri desktop app)

```bash
# Install dependencies
pnpm install

# Run web version (browser-only, mock data)
pnpm dev

# Run desktop app (full functionality)
pnpm dev:tauri
```

## Development

See [DEVELOPMENT.md](./DEVELOPMENT.md) for detailed development instructions.

### Quick Commands

| Command | Description |
|---------|-------------|
| `pnpm dev` | Start web dev server |
| `pnpm dev:tauri` | Start Tauri desktop app |
| `pnpm build:web` | Build web frontend |
| `pnpm build:tauri:mac` | Build macOS app |
| `pnpm build:tauri:win` | Build Windows app |
| `pnpm build:tauri:linux` | Build Linux app |
| `pnpm test` | Run all tests |
| `pnpm check` | Type check all |

## Project Structure

```
peterparker/
├── apps/
│   ├── web/              # SvelteKit web frontend
│   │   ├── src/
│   │   │   ├── lib/      # Components, stores, scanner client
│   │   │   └── routes/   # SvelteKit routes
│   │   └── package.json
│   └── tauri/            # Tauri desktop app
│       └── src-tauri/    # Rust backend
├── packages/
│   └── core/             # Rust core library
│       ├── src/
│       │   ├── scanner/  # Network scanning engine
│       │   ├── fingerprint/ # Vendor/OS detection
│       │   └── models/   # Data structures
│       └── data/
│           └── manuf.txt # 44,000+ MAC vendor mappings
├── Cargo.toml            # Rust workspace
├── package.json          # Root npm scripts
└── turbo.json            # Turborepo config
```

## Tech Stack

- **Frontend**: SvelteKit 5 + TypeScript + Tailwind CSS + shadcn-svelte
- **Desktop**: Tauri v2 (Rust + WebKit)
- **Backend**: Rust with Tokio async runtime
- **Network**: Raw socket operations for ARP/ICMP/TCP
- **Package Manager**: pnpm with workspaces
- **Build**: Vite + Turbo

## Architecture

### Two Modes

1. **Web Mode** (`pnpm dev`)
   - Runs in browser at `http://localhost:5173`
   - Uses mock scanner data for development
   - No system network access required
   - Good for UI development and testing

2. **Tauri Mode** (`pnpm dev:tauri`)
   - Native desktop application
   - Full network scanning capabilities
   - Cross-platform native APIs
   - Real-time IPC between frontend and backend

### Communication Flow

```
┌─────────────────────────────────────────────────────────┐
│                     Frontend (SvelteKit)                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │  Dashboard  │  │ Device List │  │  Device Detail  │  │
│  └─────────────┘  └─────────────┘  └─────────────────┘  │
│                          │                               │
│                   ┌──────────────┐                        │
│                   │ scannerClient│                        │
│                   └──────────────┘                        │
└─────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │                               │
        ┌──────────┐                    ┌──────────┐
        │   Web    │                    │  Tauri   │
        │  (Mock)  │                    │  (IPC)   │
        └──────────┘                    └──────────┘
                                              │
┌─────────────────────────────────────────────┴─────────────┐
│                    Backend (Rust/Tauri)                    │
│  ┌─────────────────────────────────────────────────────┐  │
│  │                  Scanner Engine                      │  │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌──────────┐  │  │
│  │  │ ARP     │ │ TCP     │ │ ICMP    │ │ Service  │  │  │
│  │  │ Scanner │ │ SynScan │ │ Ping    │ │ Discovery│  │  │
│  │  └─────────┘ └─────────┘ └─────────┘ └──────────┘  │  │
│  └─────────────────────────────────────────────────────┘  │
│         │                    │                    │       │
│  ┌──────────────┐   ┌─────────────┐   ┌──────────────────┐│
│  │ ARP Table    │   │ OS Finger-  │   │ Vendor Database  ││
│  │ (System)     │   │ print       │   │ (44,000+ OUIs)   ││
│  └──────────────┘   └─────────────┘   └──────────────────┘│
└───────────────────────────────────────────────────────────┘
```

## Platform Support

| Platform | Web | Tauri Dev | Tauri Build |
|----------|-----|-----------|-------------|
| macOS (Intel) | ✅ | ✅ | ✅ |
| macOS (Apple Silicon) | ✅ | ✅ | ✅ |
| Windows | ✅ | ✅ | ✅ |
| Linux | ✅ | ✅ | ✅ |

## Key Components

### Scanner Engine (`packages/core/src/scanner/`)

- **ARP Scanner** (`arp.rs`): Cross-platform ARP table parsing and MAC address discovery
- **TCP Scanner** (`tcp.rs`): TCP SYN and connect scanning for port discovery
- **HTTP Scanner** (`http.rs`): HTTP banner grabbing and title extraction
- **Engine** (`engine.rs`): Orchestrates scanning with real-time event emission

### Vendor Database

44,000+ MAC address vendor mappings loaded from Wireshark's OUI database at build time. Enables instant vendor identification from MAC addresses.

### Real-time Updates

Tauri event system provides live scan progress:
- `scan-progress`: Progress updates (scanned/total hosts, devices found)
- `device-found`: New device discovered during scan
- `scan-completed`: Scan finished successfully
- `scan-error`: Error occurred during scan

## License

MIT

## Acknowledgments

- MAC vendor database from [Wireshark](https://www.wireshark.org/download/automated/data/manuf)
- UI components from [shadcn-svelte](https://www.shadcn-svelte.com/)
- Built with [Tauri](https://tauri.app/) and [SvelteKit](https://kit.svelte.dev/)
