# PeterParker Architecture

## Overview

Two-mode architecture supporting both browser development and desktop deployment.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Presentation Layer                             │
│                         ┌─────────────────────┐                          │
│                         │   SvelteKit Web UI  │                          │
│                         └─────────────────────┘                          │
│                                    │                                     │
│              ┌─────────────────────┴─────────────────────┐               │
│              │                                           │               │
│       ┌──────────┐                                 ┌──────────┐          │
│       │  Browser │                                 │  Tauri   │          │
│       │  (Mock)  │                                 │  (Real)  │          │
│       └────┬─────┘                                 └────┬─────┘          │
│            │                                           │                │
│            │ HTTP                                      │ IPC            │
│            │                                           │                │
├────────────┴───────────────────────────────────────────┴────────────────┤
│                           Service Layer                                  │
│  ┌──────────────────┐                         ┌──────────────────┐      │
│  │   Mock Scanner   │                         │  Core Scanner    │      │
│  │   (JavaScript)   │                         │  (Rust/Tauri)    │      │
│  │                  │                         │                  │      │
│  │  • Mock devices  │                         │  • ARP scanner   │      │
│  │  • Simulated     │                         │  • TCP connect   │      │
│  │    progress      │                         │  • HTTP banner   │      │
│  │                  │                         │  • 44k vendors   │      │
│  └──────────────────┘                         └──────────────────┘      │
└─────────────────────────────────────────────────────────────────────────┘
```

## Design Decisions

### 1. Dual Mode Operation
- **Web Mode**: Mock data for UI development without Tauri
- **Tauri Mode**: Real network scanning via embedded Rust

Both modes share the same UI components and state management.

### 2. Client Abstraction

```typescript
interface ScannerClient {
  getDevices(): Promise<Device[]>;
  startScan(config: ScanConfig): Promise<string>;
  getScanProgress(id: string): Promise<ScanProgress>;
  // ... etc
}
```

Implementations:
- `TauriScannerClient` - IPC to Rust backend (real scanning)
- `MockScannerClient` - Simulated data (browser dev)

### 3. Real-time Updates

Tauri mode uses event system for live updates:
```
Rust Engine → Tauri Event → Frontend Listener → UI Update
```

Events:
- `scan-progress` - Progress bar updates
- `device-found` - Add device to list immediately
- `scan-completed` - Scan finished
- `scan-error` - Error occurred

### 4. Build Targets

| Target | Frontend | Backend | Command |
|--------|----------|---------|---------|
| Web | SvelteKit | Mock | `pnpm dev` |
| Tauri | SvelteKit | Core Library | `pnpm dev:tauri` |

## Data Flow

### Browser Mode (Mock)
```
User Action → UI Component → MockScannerClient → Simulated Data → UI Update
```

### Tauri Mode (Real)
```
User Action → UI Component → TauriScannerClient → IPC → Rust Engine → Network
                     ↑______________________________________________↓
                              (Tauri Events for real-time updates)
```

## Package Structure

```
peterparker/
├── apps/
│   ├── web/               # SvelteKit frontend
│   │   └── src/lib/
│   │       ├── components/    # UI components
│   │       ├── stores/        # State management
│   │       └── scanner/       # Client abstraction
│   └── tauri/             # Tauri desktop wrapper
│       └── src-tauri/
│           └── src/
│               ├── commands/  # IPC handlers
│               └── lib.rs
├── packages/
│   └── core/              # Rust core library
│       └── src/
│           ├── scanner/     # Engine + ARP + TCP + HTTP
│           ├── fingerprint/ # Vendor + OS detection
│           └── models/      # Data structures
└── Cargo.toml             # Rust workspace
```

## Development Workflow

```bash
# Web development (mock data)
pnpm dev

# Desktop development (real scanning)
pnpm dev:tauri

# Build for production
pnpm build:tauri:mac     # macOS Universal
pnpm build:tauri:win     # Windows
pnpm build:tauri:linux   # Linux
```

## Key Components

### Scanner Engine (`packages/core/src/scanner/`)
- **Engine** (`engine.rs`) - Orchestrates scanning with event emission
- **ARP** (`arp.rs`) - Cross-platform ARP table parsing
- **TCP** - TCP connect scanning for ports
- **HTTP** - Banner grabbing and title extraction

### Vendor Database
44,000+ MAC vendor mappings from Wireshark, embedded at compile time.

### Frontend State
- **scanStore** - Manages scan lifecycle, listens to Tauri events
- **devicesStore** - Device list management
- **settingsStore** - User preferences

## Deployment

### Desktop (Tauri)
- Single binary with embedded web assets
- Real network scanning via native Rust
- Cross-platform: macOS, Windows, Linux

### Web (Browser)
- Static build for hosting
- Mock scanner data only
- Demo/development purposes
