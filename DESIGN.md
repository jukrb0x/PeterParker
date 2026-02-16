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
                              ▼ IPC (Tauri Commands)
┌─────────────────────────────────────────────────────────────┐
│                      Backend (Rust)                          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Scanner Engine                      │   │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌───────────┐  │   │
│  │  │ ARP     │ │ TCP     │ │ ICMP    │ │ Service   │  │   │
│  │  │ Scanner │ │ SynScan │ │ Ping    │ │ Discovery │  │   │
│  │  └─────────┘ └─────────┘ └─────────┘ └───────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│         │                    │                    │         │
│  ┌──────────────┐   ┌─────────────┐   ┌──────────────────┐ │
│  │ Device Cache │   │ OS Fingerprint│  │ Protocol Parser  │ │
│  │ (SQLite/DashMap)│ │ Engine      │   │ (HTTP/SMB/mDNS)  │ │
│  └──────────────┘   └─────────────┘   └──────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 2.3 Module Structure

```
peterparker/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Library exports
│   │   ├── commands/            # Tauri IPC handlers
│   │   │   ├── mod.rs
│   │   │   ├── scan.rs          # Scan commands
│   │   │   └── device.rs        # Device CRUD
│   │   ├── scanner/             # Core scanner engine
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs        # Scan orchestrator
│   │   │   ├── arp.rs           # ARP discovery
│   │   │   ├── tcp.rs           # TCP SYN scanning
│   │   │   ├── icmp.rs          # ICMP ping sweeps
│   │   │   └── service.rs       # Service detection
│   │   ├── fingerprint/         # Device fingerprinting
│   │   │   ├── mod.rs
│   │   │   ├── os.rs            # OS detection
│   │   │   ├── vendor.rs        # MAC → vendor lookup
│   │   │   └── http.rs          # HTTP header analysis
│   │   ├── models/              # Data structures
│   │   │   ├── mod.rs
│   │   │   ├── device.rs
│   │   │   ├── port.rs
│   │   │   └── scan.rs
│   │   └── db/                  # Local storage
│   │       ├── mod.rs
│   │       └── sqlite.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/
│   ├── app.html
│   ├── app.d.ts
│   ├── lib/
│   │   ├── components/          # shadcn-svelte + custom
│   │   │   ├── ui/              # shadcn base
│   │   │   ├── device/          # Device card, detail
│   │   │   ├── scan/            # Progress, controls
│   │   │   └── layout/          # Nav, sidebar
│   │   ├── stores/              # Svelte 5 runes
│   │   │   ├── devices.svelte.ts
│   │   │   ├── scan.svelte.ts
│   │   │   └── settings.svelte.ts
│   │   ├── scanner/             # Frontend scanner API
│   │   │   ├── client.ts        # Tauri invoke wrapper
│   │   │   └── types.ts         # Shared types
│   │   └── utils/
│   ├── routes/
│   │   ├── +layout.svelte
│   │   ├── +page.svelte         # Dashboard
│   │   └── device/[ip]/+page.svelte
│   └── styles/
├── tests/                       # vitest tests
├── docs/
└── package.json
```

---

## 3. Data Models

### 3.1 Core Types (Rust/TS shared)

```typescript
// Device - central entity
interface Device {
  id: string;                    // ULID
  ip: string;                    // IPv4/v6
  mac: string | null;            // MAC address
  vendor: string | null;         // MAC vendor lookup
  hostname: string | null;       // DNS/mDNS hostname
  os: OperatingSystem | null;    // Detected OS
  deviceType: DeviceType;        // Classified type
  firstSeen: Date;
  lastSeen: Date;
  isOnline: boolean;
  ports: Port[];
  metadata: DeviceMetadata;
}

// Port scan result
interface Port {
  number: number;
  protocol: 'tcp' | 'udp';
  state: 'open' | 'closed' | 'filtered';
  service: Service | null;
  banner: string | null;
}

// Service fingerprint
interface Service {
  name: string;                  // ssh, http, etc
  version: string | null;
  product: string | null;
  extraInfo: Record<string, unknown>;
}

// OS detection
interface OperatingSystem {
  name: string;
  family: 'windows' | 'linux' | 'macos' | 'bsd' | 'embedded' | 'unknown';
  version: string | null;
  confidence: number;            // 0-100
  cpe: string[];
}

// Device classification
enum DeviceType {
  Router = 'router',
  Switch = 'switch',
  Desktop = 'desktop',
  Laptop = 'laptop',
  Mobile = 'mobile',
  Iot = 'iot',
  Printer = 'printer',
  Nas = 'nas',
  Unknown = 'unknown'
}

// Scan configuration
interface ScanConfig {
  targetRange: string;           // CIDR notation
  ports: 'top100' | 'top1000' | number[] | 'all';
  scanType: 'arp' | 'ping' | 'syn' | 'connect' | 'comprehensive';
  timeout: number;               // ms per host
  concurrency: number;           // parallel hosts
  enableOsDetection: boolean;
  enableServiceDetection: boolean;
}

// Scan progress
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

### 4.2 Key Screens

#### Dashboard (`/`)
```
┌─────────────────────────────────────────────────────────┐
│  PeterParker                              [Scan ▼] [⚙️] │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────┐  ┌─────────────────┐              │
│  │   12 Devices    │  │   3 New Today   │              │
│  │   8 Online      │  │   2 Offline     │              │
│  └─────────────────┘  └─────────────────┘              │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Network Graph (D3/Force)                         │   │
│  │ • Nodes = devices                                │   │
│  │ • Edges = traffic paths (simulated)              │   │
│  │ • Color = device type                            │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  Recent Activity                                        │
│  ─────────────────                                      │
│  • 192.168.1.105  joined  [2 min ago]                  │
│  • 192.168.1.109  left    [5 min ago]                  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

#### Device List
```
┌─────────────────────────────────────────────────────────┐
│  [🔍 Filter...]  [Router ▼]  [Online ▼]  [Grid/List]   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 🌐  192.168.1.1          TP-LINK Router         │   │
│  │     00:1A:2B:3C:4D:5E    ● Online 2d            │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 🖥️  192.168.1.101        John's MacBook Pro    │   │
│  │     00:1A:2B:3C:4D:5F    ● Online 5min          │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 📱  192.168.1.106        iPhone 15             │   │
│  │     00:1A:2B:3C:4D:60    ○ Offline 2h           │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

#### Device Detail
```
┌─────────────────────────────────────────────────────────┐
│  ← Back                        [Export] [Edit] [🗑️]     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  🌐  TP-LINK Router                                     │
│      192.168.1.1                              ● Online  │
│      00:1A:2B:3C:4D:5E                                  │
│                                                         │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────┐ │
│  │ OS: Linux     │ │ Type: Router  │ │ First: 2w ago │ │
│  └───────────────┘ └───────────────┘ └───────────────┘ │
│                                                         │
│  Open Ports                                  [Rescan]   │
│  ─────────────────────────────────────────────────────  │
│  Port  │ Service    │ Version       │ Banner            │
│  80/tcp  http         nginx/1.20      Server: nginx    │
│  53/udp  dns          dnsmasq 2.86                      │
│  22/tcp  ssh          OpenSSH 8.9    SSH-2.0-OpenSSH   │
│                                                         │
│  Timeline                                               │
│  ─────────────────────────────────────────────────────  │
│  [Timeline: uptime/offline events over time]            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 4.3 Component Inventory

**shadcn-svelte Base**:
- Button, Card, Dialog, DropdownMenu, Input, Label, Progress, Select, Separator, Sheet, Skeleton, Switch, Table, Tabs, Tooltip

**Custom Components**:
- `DeviceCard` - Summary card with icon, IP, status
- `DeviceIcon` - Dynamic icon based on device type
- `PortTable` - Sortable port/service table
- `NetworkGraph` - D3 force-directed graph
- `ScanProgress` - Animated progress with ETA
- `StatusBadge` - Online/offline indicator
- `MacLookup` - Vendor lookup display

---

## 5. Scanner Engine

### 5.1 Scanning Strategies

| Method | Speed | Accuracy | Use Case |
|--------|-------|----------|----------|
| **ARP** | <1s | MAC only | Same subnet, fast discovery |
| **ICMP Ping** | ~2s | Reachability | Cross-subnet, router-aware |
| **TCP SYN** | ~5s | Port state | Stealth, no full handshake |
| **TCP Connect** | ~10s | Full service | Most compatible |
| **Comprehensive** | ~60s | Maximum | Full fingerprint |

### 5.2 Rust Scanner Architecture

```rust
// Core traits for scanner modules
pub trait Scanner: Send + Sync {
    fn name(&self) -> &'static str;
    fn scan(&self, target: &Target) -> impl Future<Output = Result<ScanResult>>;
}

// Engine orchestrates multiple scanners
pub struct ScannerEngine {
    scanners: Vec<Box<dyn Scanner>>,
    concurrency: usize,
    timeout: Duration,
}

impl ScannerEngine {
    pub async fn scan_range(&self, range: IpRange) -> mpsc::Receiver<Device> {
        // Parallel execution with backpressure
    }
}
```

### 5.3 Key Features

1. **ARP Scanning**: Raw socket manipulation, cache snooping
2. **TCP SYN**: Custom socket options for stealth
3. **Service Detection**: Banner grabbing + probe matching
4. **OS Fingerprinting**: TCP/IP stack quirks analysis
5. **mDNS/LLMNR**: Hostname resolution beyond DNS

---

## 6. Testing Strategy

### 6.1 Frontend (vitest)

```typescript
// tests/components/DeviceCard.test.ts
import { render, screen } from '@testing-library/svelte';
import DeviceCard from '$lib/components/device/DeviceCard.svelte';

describe('DeviceCard', () => {
  const mockDevice = {
    ip: '192.168.1.1',
    mac: '00:11:22:33:44:55',
    vendor: 'Apple',
    isOnline: true,
    deviceType: DeviceType.Router
  };

  it('renders device IP', () => {
    render(DeviceCard, { props: { device: mockDevice } });
    expect(screen.getByText('192.168.1.1')).toBeInTheDocument();
  });

  it('shows online status', () => {
    render(DeviceCard, { props: { device: mockDevice } });
    expect(screen.getByTestId('status-online')).toBeVisible();
  });
});

// tests/stores/devices.test.ts
import { describe, it, expect } from 'vitest';
import { createDevicesStore } from '$lib/stores/devices.svelte';

describe('devices store', () => {
  it('adds device to store', () => {
    const store = createDevicesStore();
    const device = { id: '1', ip: '192.168.1.1' };
    store.add(device);
    expect(store.getAll()).toHaveLength(1);
  });
});
```

### 6.2 Backend (cargo test)

```rust
// src/scanner/arp.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_arp_scan_localhost() {
        let scanner = ArpScanner::new();
        let target = Target::new("127.0.0.1").unwrap();
        let result = scanner.scan(&target).await;
        assert!(result.is_ok());
    }
}

// Integration tests
// tests/scanner_integration.rs
#[tokio::test]
async fn test_end_to_end_scan() {
    let engine = ScannerEngine::builder()
        .add_scanner(ArpScanner::new())
        .add_scanner(IcmpScanner::new())
        .concurrency(10)
        .build();
    
    let range = IpRange::parse("192.168.1.0/24").unwrap();
    let devices: Vec<_> = engine.scan_range(range).collect().await;
    
    assert!(!devices.is_empty());
}
```

---

## 7. Milestones

### Phase 1: Foundation (Week 1)
- [ ] Project scaffold (Tauri + SvelteKit + shadcn)
- [ ] Git setup, CI/CD skeleton
- [ ] Core Rust models + types
- [ ] Basic ARP scanner

### Phase 2: Core Features (Week 2)
- [ ] TCP SYN scanner
- [ ] ICMP ping sweeps
- [ ] Device list UI
- [ ] Real-time scan progress

### Phase 3: Intelligence (Week 3)
- [ ] MAC vendor lookup
- [ ] OS fingerprinting
- [ ] Service detection
- [ ] Device detail view

### Phase 4: Polish (Week 4)
- [ ] Network graph visualization
- [ ] Export (JSON/CSV)
- [ ] Settings persistence
- [ ] Tests + documentation

---

## 8. Security Considerations

1. **Raw sockets**: Requires elevated privileges on some platforms
2. **Network traffic**: Scanner generates noticeable traffic
3. **Data storage**: Device cache stored locally, encrypted at rest
4. **Permissions**: Explicit user consent for network access

---

## 9. File Structure

```
peterparker/
├── AGENTS.md                    # This file
├── DESIGN.md                    # Design specification
├── README.md                    # User documentation
├── LICENSE                      # MIT
├── .gitignore
├── package.json                 # Node dependencies
├── svelte.config.js
├── vite.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── components.json              # shadcn-svelte config
├── src/
│   ├── app.html
│   ├── app.d.ts
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/              # shadcn components
│   │   │   ├── device/
│   │   │   ├── scan/
│   │   │   └── layout/
│   │   ├── stores/
│   │   ├── scanner/
│   │   └── utils/
│   ├── routes/
│   └── styles/
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   ├── icons/
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands/
│       ├── scanner/
│       ├── fingerprint/
│       ├── models/
│       └── db/
├── tests/                       # vitest tests
└── docs/                        # Additional docs
```

---

## 10. Commands

```bash
# Setup
cd peterparker
npm install
cd src-tauri && cargo build

# Dev
npm run tauri dev           # Start dev server

# Test
npm run test               # vitest
npm run test:ui            # vitest UI
cd src-tauri && cargo test # Rust tests

# Build
npm run tauri build        # Production build
```
