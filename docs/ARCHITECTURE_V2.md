# PeterParker Architecture v2

## Overview

Three-layer architecture separating concerns for maximum reusability across platforms.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Presentation Layer                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐  │
│  │   Web App    │  │ Tauri App    │  │  Mobile App  │  │  CLI Tool   │  │
│  │  (Browser)   │  │  (Desktop)   │  │(React Native│  │  (Terminal) │  │
│  │              │  │              │  │/Swift/etc)  │  │             │  │
│  │ HTTP/WebSocket│  │ Direct API   │  │ HTTP/WS      │  │  Direct API │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬──────┘  │
│         │                 │                   │                 │        │
│         └─────────────────┴───────────────────┴─────────────────┘        │
│                                    │                                     │
│                         ┌──────────┴──────────┐                         │
│                         │   Shared UI Kit     │                         │
│                         │  (Svelte Components)│                         │
│                         └─────────────────────┘                         │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────────────┐
│                           Service Layer                                  │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                  Scanner Service (Rust)                          │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │   │
│  │  │  HTTP API   │  │ WebSocket   │  │   Core Scanner Engine   │ │   │
│  │  │  (REST/JSON)│  │ (Realtime)  │  │  - TCP/ICMP/ARP scan    │ │   │
│  │  └─────────────┘  └─────────────┘  │  - OS fingerprinting    │ │   │
│  │                                    │  - Service detection    │ │   │
│  │  ┌─────────────────────────────────┤  - MAC vendor lookup    │ │   │
│  │  │         Scanner Core Library     │  - HTTP banner grab    │ │   │
│  │  │    (peterparker-core crate)      └─────────────────────────┘ │   │
│  │  └─────────────────────────────────────────────────────────────┘   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Scanner Service is Standalone
- Runs as separate process (localhost:3030)
- Can be deployed independently
- Language-agnostic API (HTTP + WebSocket)

### 2. Frontend Adapters
Each platform implements the same `ScannerClient` interface:

```typescript
interface ScannerClient {
  getDevices(): Promise<Device[]>;
  startScan(config: ScanConfig): Promise<string>;
  getScanProgress(id: string): Promise<ScanProgress>;
  // ... etc
}
```

Implementations:
- `HttpScannerClient` - Web, Mobile, CLI
- `TauriScannerClient` - Direct Rust call
- `WsScannerClient` - Real-time updates

### 3. Shared UI Components
Svelte components work everywhere:
- Web: Direct render
- Tauri: Direct render
- Mobile: Svelte Native or WebView

### 4. Build Targets
| Target | Frontend | Backend Connection | Build Command |
|--------|----------|-------------------|---------------|
| Web | SvelteKit | HTTP/WebSocket to Scanner Service | `pnpm build:web` |
| Tauri | SvelteKit | Direct Rust API (embedded) | `pnpm build:tauri` |
| Mobile | React Native / Swift | HTTP/WebSocket to Scanner Service | Future |

## Data Flow

### Web / Mobile
```
User Action → UI Component → HttpScannerClient → HTTP → Scanner Service → Core Library → Network
                     ↑______________________________________________↓
                              (Response / WebSocket update)
```

### Tauri
```
User Action → UI Component → TauriScannerClient → Rust Command → Core Library → Network
                     ↑_____________________________________________________↓
                                    (Direct response)
```

## API Specification

### HTTP Endpoints
```
GET    /api/devices              List all devices
GET    /api/devices/:ip          Get device by IP
POST   /api/scan                 Start scan
GET    /api/scan/:id             Get scan progress/result
DELETE /api/devices/:id          Delete device
```

### WebSocket Messages
```typescript
// Client → Server
{ type: "ping" }
{ type: "subscribe_devices" }
{ type: "start_scan", config: ScanConfig }

// Server → Client  
{ type: "pong" }
{ type: "devices_updated", devices: Device[] }
{ type: "scan_progress", scanId: string, progress: ScanProgress }
```

## Package Structure

```
peterparker/
├── apps/
│   ├── scanner/           # Rust HTTP/WebSocket service
│   ├── web/               # SvelteKit web app (HTTP client)
│   └── tauri/             # Tauri desktop (Direct Rust)
├── packages/
│   ├── core/              # Rust core library
│   └── ui/                # Shared Svelte components (optional)
└── turbo.json
```

## Development Workflow

```bash
# Terminal 1: Start Scanner Service
pnpm dev:scanner

# Terminal 2: Start Web App (connects to scanner)
pnpm dev:web

# OR: Start Tauri (embedded, no separate scanner needed)
pnpm dev:tauri
```

## Deployment Options

### Option 1: Full Stack (Recommended for desktop)
- Scanner Service + Tauri frontend
- Everything bundled together

### Option 2: Client-Server (For web/mobile)
- Scanner Service on server/VPS
- Web app connects remotely

### Option 3: Hybrid
- Scanner Service on local network device (Raspberry Pi)
- Web/Mobile apps connect to it
