# PeterParker

> Your Friendly Neighborhood Network Scanner

A cross-platform LAN device scanner with detailed device fingerprinting.

## Features

- 🔍 **Network Discovery** - Fast scanning of local network devices
- 📊 **Device Fingerprinting** - OS detection, vendor lookup, service identification
- 🎯 **Port Scanning** - TCP SYN/Connect scans with service detection
- 🌐 **Beautiful UI** - Dark theme, Jony Ive-inspired minimalism
- 💾 **Persistent Storage** - SQLite database for scan history
- 📤 **Export** - JSON/CSV export of discovered devices

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop | Tauri v2 |
| Frontend | SvelteKit + TypeScript |
| UI | shadcn-svelte + Tailwind CSS |
| Backend | Rust |
| Testing | vitest (frontend), cargo test (Rust) |

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.75+
- Platform-specific dependencies (see [Tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites))

### Installation

```bash
# Clone the repository
git clone https://github.com/jabriel/peterparker.git
cd peterparker

# Install dependencies
npm install

# Run in development
npm run tauri:dev
```

### Building

```bash
npm run tauri:build
```

## Project Structure

```
peterparker/
├── src/                    # SvelteKit frontend
│   ├── lib/
│   │   ├── components/    # UI components
│   │   ├── stores/        # Svelte 5 runes state
│   │   ├── scanner/       # Scanner client API
│   │   └── utils/         # Utility functions
│   └── routes/            # SvelteKit routes
├── src-tauri/             # Rust backend
│   └── src/
│       ├── commands/      # Tauri IPC handlers
│       ├── scanner/       # Scanner engine
│       ├── fingerprint/   # OS/vendor detection
│       └── models/        # Data structures
└── tests/                 # vitest tests
```

## Development

```bash
# Run frontend dev server
npm run dev

# Run Tauri dev
npm run tauri:dev

# Run tests
npm run test

# Run Rust tests
cd src-tauri && cargo test
```

## Architecture

### Scanner Engine

The Rust scanner engine supports multiple scan methods:

| Method | Speed | Use Case |
|--------|-------|----------|
| ARP | <1s | Same subnet, MAC discovery |
| ICMP | ~2s | Cross-subnet reachability |
| TCP SYN | ~5s | Stealth port scanning |
| TCP Connect | ~10s | Service detection |

### Data Flow

```
Scanner Engine (Rust)
    ↓ Tauri IPC
State Stores (Svelte 5 runes)
    ↓ Reactive
UI Components (Svelte)
```

## License

MIT

---

Made with ❤️ by Jabriel
