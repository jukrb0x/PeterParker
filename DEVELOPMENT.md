# PeterParker Development Guide

## Quick Start

```bash
# Install dependencies
pnpm install

# Run web version (browser-only with mock data)
pnpm dev
# or
pnpm dev:web

# Run Tauri desktop app (full functionality)
pnpm dev:tauri
```

## Monorepo Structure

```
peterparker/
├── apps/
│   ├── web/              # SvelteKit web frontend
│   └── tauri/            # Tauri desktop app (embeds web)
├── packages/
│   └── core/             # Rust core library (scanner engine)
├── Cargo.toml            # Rust workspace
├── package.json          # Root npm scripts
└── turbo.json            # Turborepo configuration
```

## Development Modes

### 1. Web Development (Browser Only)

Best for quick UI iteration. Uses mock scanner data.

```bash
pnpm dev
# or
pnpm dev:web
```

- URL: http://localhost:5173
- Scanner: Mock data
- Hot reload: ✅

### 2. Tauri Development (Full Desktop App)

Full functionality with real network scanning.

```bash
pnpm dev:tauri
```

- Requires Rust toolchain
- Real network scanning via Rust backend
- IPC communication via Tauri APIs

## Building

### Web Build

```bash
pnpm build:web
```

Output: `apps/web/build/`

### Tauri Builds

```bash
# macOS Universal (Intel + Apple Silicon)
pnpm build:tauri:mac

# Windows
pnpm build:tauri:win

# Linux
pnpm build:tauri:linux

# Generic (detects current platform)
pnpm build:tauri
```

Outputs: `apps/tauri/src-tauri/target/release/bundle/`

## Testing

```bash
# Run all tests
pnpm test

# Web tests only
pnpm test:web

# Rust tests only
pnpm test:core

# With coverage
pnpm test:web -- --coverage
```

## Rust Development

```bash
# Check Rust code
cargo check

# Build Rust packages
cargo build

# Build release
cargo build --release

# Format Rust code
cargo fmt

# Run Rust tests
cargo test
```

## Platform-Specific Notes

### macOS

For local development on Apple Silicon:

```bash
# Install Rust targets
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Run dev
pnpm dev:tauri

# Build universal binary
pnpm build:tauri:mac
```

### Windows

Requires Visual Studio Build Tools or Visual Studio Community with C++ workload.

```bash
# Build for Windows
pnpm build:tauri:win
```

### Linux

Requires system dependencies:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libappindicator3-dev \
  librsvg2-dev

# Build
pnpm build:tauri:linux
```

## Environment Variables

Create `.env` files in respective app directories:

### apps/web/.env

```env
# No special env vars needed for web
```

### apps/tauri/.env

```env
# Tauri-specific config
TAURI_DEBUG=1
```

## Troubleshooting

### Tauri dev not starting

1. Check Rust installation: `rustc --version`
2. Check Tauri CLI: `cargo tauri --version`
3. Delete `target/` directory and retry

### Web build fails

```bash
# Clean and reinstall
pnpm run clean:node
pnpm install
```

### Rust compilation errors

```bash
# Clean Rust builds
pnpm run clean:rust

# Update dependencies
cargo update
```

## Scripts Reference

| Script | Description |
|--------|-------------|
| `pnpm dev` | Start web dev server |
| `pnpm dev:tauri` | Start Tauri dev app |
| `pnpm build` | Build all packages |
| `pnpm build:web` | Build web frontend |
| `pnpm build:tauri:mac` | Build macOS app |
| `pnpm build:tauri:win` | Build Windows app |
| `pnpm build:tauri:linux` | Build Linux app |
| `pnpm test` | Run all tests |
| `pnpm test:core` | Run Rust tests |
| `pnpm test:web` | Run web tests |
| `pnpm check` | Type check all |
| `pnpm lint` | Lint all |
| `pnpm format` | Format all |
| `pnpm clean` | Clean all builds |

## Architecture Notes

### Web Mode
- Runs in browser
- Uses mock scanner data
- No system network access
- Good for UI development

### Tauri Mode
- Desktop application
- Native Rust backend
- Real network scanning
- Cross-platform native APIs

### Communication
- Web → Tauri: Uses `@tauri-apps/api` invoke()
- Tauri → Web: Uses Tauri event system
- Mock → Web: Direct function calls with mock data
