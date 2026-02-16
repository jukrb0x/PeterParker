# PeterParker - Design Specification

## 1. Overview

**App Name**: PeterParker  
**Tagline**: "Your Friendly Neighborhood Network Scanner"  
**Version**: 0.1.0

**Purpose**: Fast, accurate LAN device discovery with detailed fingerprinting and beautiful data visualization.

---

## 2. Architecture

### 2.1 Stack Rationale

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Desktop | **Tauri v2** | 600KB vs 150MB (Electron), Rust-native, secure, mobile roadmap |
| UI | **SvelteKit** | Compiler-based reactivity, minimal runtime, elegant syntax |
| Styling | **shadcn-svelte + Tailwind** | Accessible, composable, customizable |
| Backend | **Rust** | Zero-cost async, memory safety, perfect for network I/O |
| Testing | **vitest + cargo test** | Fast, native, comprehensive |

### 2.2 System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Frontend (SvelteKit)                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Dashboard  │  │ Device List │  │ Device Detail View  │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│         │                │                    │               │
│         └────────────────┴────────────────────┘               │
│                          │                                    │
│                   ┌──────────────┐                           │
│                   │ State Stores │ (Svelte 5 runes)          │
│                   └──────────────┘                           │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │                               │
        ┌──────────┐                    ┌──────────┐
        │   Web    │                    │  Tauri   │
        │  (Mock)  │                    │  (Real)  │
        └──────────┘                    └──────────┘
                                              │
┌─────────────────────────────────────────────┴─────────────┐
│                    Backend (Rust/Tauri)                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Scanner Engine                      │   │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌───────────┐  │   │
│  │  │ ARP     │ │ TCP     │ │ ICMP    │ │ HTTP      │  │   │
│  │  │ Scanner │ │ Connect │ │ Ping    │ │ Banner    │  │   │
│  │  └─────────┘ └─────────┘ └─────────┘ └───────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│         │                    │                    │         │
│  ┌──────────────┐   ┌─────────────┐   ┌──────────────────┐ │
│  │ ARP Table    │   │ OS Finger-  │   │ Vendor Database  │ │
│  │ (System)     │   │ print       │   │ (44,000+ OUIs)   │ │
│  └──────────────┘   └─────────────┘   └──────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 2.3 Module Structure

```
peterparker/
├── apps/
│   ├── web/           # SvelteKit frontend (@peterparker/web)
│   └── tauri/         # Tauri desktop app
│       └── src-tauri/
│           ├── src/
│           │   ├── commands/      # Tauri IPC handlers
│           │   ├── lib.rs         # App setup
│           │   └── main.rs        # Entry point
│           └── Cargo.toml
├── packages/
│   └── core/          # Rust core library (peterparker-core)
│       ├── src/
│       │   ├── scanner/       # Engine, ARP, TCP, HTTP
│       │   ├── fingerprint/   # Vendor, OS detection
│       │   └── models/        # Device, Port, Scan types
│       └── data/
│           └── manuf.txt      # Wireshark vendor DB
├── Cargo.toml         # Rust workspace
├── package.json       # npm scripts
├── turbo.json         # Turbo config
└── pnpm-workspace.yaml
```

---

## 3. Data Models

### 3.1 Core Types (Rust/TS shared)

```typescript
interface Device {
  id: string;                    // UUID
  ip: string;                    // IPv4
  mac: string | null;            // MAC address
  vendor: string | null;         // MAC vendor lookup
  hostname: string | null;       // DNS/mDNS hostname
  os: OperatingSystem | null;    // Detected OS
  deviceType: DeviceType;        // Classified type
  firstSeen: string;             // ISO date
  lastSeen: string;              // ISO date
  isOnline: boolean;
  ports: Port[];
  metadata: DeviceMetadata;
}

interface Port {
  number: number;
  protocol: 'tcp' | 'udp';
  state: 'open' | 'closed' | 'filtered';
  service: Service | null;
  banner: string | null;
}

interface Service {
  name: string;
  version: string | null;
  product: string | null;
  extraInfo: Record<string, unknown>;
}

interface OperatingSystem {
  name: string;
  family: 'windows' | 'linux' | 'macos' | 'bsd' | 'embedded' | 'unknown';
  version: string | null;
  confidence: number;
  cpe: string[];
}

interface ScanConfig {
  targetRange: string;           // CIDR notation
  ports: 'top100' | 'top1000' | number[] | 'all';
  scanType: 'arp' | 'ping' | 'syn' | 'connect' | 'comprehensive';
  timeout: number;
  concurrency: number;
  enableOsDetection: boolean;
  enableServiceDetection: boolean;
}

interface ScanProgress {
  scanId: string;
  status: 'pending' | 'running' | 'paused' | 'completed' | 'error';
  totalHosts: number;
  scannedHosts: number;
  foundDevices: number;
  currentHost: string | null;
  eta: number | null;
  error: string | null;
}
```

---

## 4. UI Design

### 4.1 Visual Language

**Principles**: Clarity, deference, depth (iOS-inspired minimalism)

**Colors**:
- Background: `#0a0a0a` (deep black)
- Surface: `#141414` (elevated)
- Surface highlight: `#1a1a1a`
- Primary: `#3b82f6` (blue)
- Success: `#22c55e` (green)
- Warning: `#f59e0b` (amber)
- Error: `#ef4444` (red)
- Text primary: `#fafafa`
- Text secondary: `#a1a1aa`

**Typography**:
- Font: Inter (system fallback)
- Sizes: 12px caption, 14px body, 16px emphasis, 20px title, 24px header
- Weights: 400 regular, 500 medium, 600 semibold, 700 bold

**Spacing Scale**: 4px base
- xs: 4px, sm: 8px, md: 16px, lg: 24px, xl: 32px, 2xl: 48px

### 4.2 Component Inventory

**shadcn-svelte Base**:
- Button, Card, Dialog, DropdownMenu, Input, Label, Progress, Select, Separator, Sheet, Skeleton, Switch, Table, Tabs, Tooltip

**Custom Components**:
- `DeviceCard` - Summary card with icon, IP, status
- `DeviceIcon` - Dynamic icon based on device type
- `ScanProgressPanel` - Animated progress with ETA
- `StatusBadge` - Online/offline indicator

---

## 5. Scanner Engine

### 5.1 Scanning Strategies

| Method | Speed | Accuracy | Use Case |
|--------|-------|----------|----------|
| **ARP** | <1s | MAC only | Same subnet, fast discovery |
| **ICMP Ping** | ~2s | Reachability | Cross-subnet, router-aware |
| **TCP Connect** | ~5s | Port state | Most compatible |
| **Comprehensive** | ~30s | Maximum | Full fingerprint |

### 5.2 Key Features

1. **ARP Scanning**: Cross-platform ARP table parsing (macOS/Linux/Windows)
2. **TCP Connect**: Port scanning with connection timeouts
3. **MAC Vendor Lookup**: 44,000+ vendor mappings from Wireshark
4. **HTTP Banner Grabbing**: Extract title and server headers
5. **Real-time Events**: Progress updates via Tauri event system

### 5.3 Event System

```rust
pub enum ScanEvent {
    Progress { scanned, total, found, current },
    DeviceFound(Device),
    Completed,
    Error(String),
}
```

Frontend listens via Tauri events:
- `scan-progress`
- `device-found`
- `scan-completed`
- `scan-error`

---

## 6. Testing Strategy

### 6.1 Frontend (vitest)

```typescript
// Component tests
import { render, screen } from '@testing-library/svelte';
import DeviceCard from '$lib/components/device/DeviceCard.svelte';

describe('DeviceCard', () => {
  it('renders device IP', () => {
    render(DeviceCard, { props: { device: mockDevice } });
    expect(screen.getByText('192.168.1.1')).toBeInTheDocument();
  });
});
```

### 6.2 Backend (cargo test)

```rust
#[tokio::test]
async fn test_arp_table_parsing() {
    let table = arp::get_arp_table().await.unwrap();
    assert!(!table.is_empty());
}

#[test]
fn test_vendor_lookup() {
    let vendor = lookup_vendor("001B63"); // Apple
    assert!(vendor.is_some());
}
```

---

## 7. Commands

```bash
# Setup
pnpm install

# Development
pnpm dev              # Web with mock data
pnpm dev:tauri        # Desktop with real scanning

# Building
pnpm build:web        # Build web frontend
pnpm build:tauri:mac  # Build macOS app
pnpm build:tauri:win  # Build Windows app
pnpm build:tauri:linux # Build Linux app

# Testing
pnpm test             # All tests
pnpm test:core        # Rust tests
pnpm test:web         # Web tests

# Maintenance
pnpm check            # Type check all
pnpm lint             # Lint all
pnpm format           # Format all
pnpm clean            # Clean builds
```

---

## 8. Security Considerations

1. **Raw sockets**: Uses system commands (ping, arp) - no root required on most platforms
2. **Network traffic**: Scanner generates ICMP and TCP traffic on local network
3. **Data storage**: Device cache in memory only (SQLite persistence TODO)
4. **Permissions**: User consent required for network access
