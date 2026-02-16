# PeterParker - LAN Scanner App

## Mission
Cross-platform LAN device scanner with detailed device fingerprinting.

## Tech Stack
- **Desktop**: Tauri v2 (Rust + WebView)
- **UI**: TypeScript + SvelteKit 5 + shadcn-svelte + Tailwind CSS
- **Backend**: Rust (peterparker-core crate)
- **Testing**: vitest (frontend), cargo test (Rust)

## Design Philosophy
- **Jony Ive minimalism**: Clean surfaces, purposeful whitespace, subtle depth
- **Radical efficiency**: No wasted motion, every pixel serves function
- **Information density**: Rich data, elegant presentation

## Architecture Principles
- Embedded scanner: Core library compiled into Tauri binary (no separate service)
- Reactive data flow: Svelte 5 runes → derived state → component bindings
- Real-time events: Tauri IPC events for scan progress updates
- Type safety: End-to-end TypeScript + Rust strict types

## Project Structure
```
peterparker/
├── apps/
│   ├── web/              # SvelteKit frontend (can run standalone with mock data)
│   │   └── src/lib/
│   │       ├── components/   # shadcn-svelte + custom
│   │       ├── stores/       # Svelte 5 reactive stores
│   │       └── scanner/      # Client API (Tauri or mock)
│   └── tauri/            # Tauri desktop app
│       └── src-tauri/
│           ├── src/
│           │   ├── commands/  # Tauri IPC handlers
│           │   └── lib.rs
│           └── Cargo.toml
├── packages/
│   └── core/             # Rust core library
│       ├── src/
│       │   ├── scanner/      # Scanner engine (ARP/TCP/HTTP)
│       │   ├── fingerprint/  # Vendor/OS detection
│       │   └── models/       # Data structures
│       └── data/
│           └── manuf.txt     # 44,000+ MAC vendor mappings
├── docs/                 # Documentation
├── Cargo.toml            # Rust workspace
├── package.json          # npm scripts
└── turbo.json            # Turborepo config
```

## Development Modes

### Browser Mode (Mock Data)
```bash
pnpm dev          # Runs web app with mock scanner data
```
- URL: http://localhost:5173
- Uses MockScannerClient
- No network access required
- Good for UI development

### Tauri Mode (Real Scanning)
```bash
pnpm dev:tauri    # Runs desktop app with real network scanning
```
- Uses TauriScannerClient with IPC
- Real ARP/TCP/HTTP scanning
- Cross-platform network access

## Development Rules
1. No placeholder code. Every function has purpose.
2. Tests required: unit for logic, integration for scanner API
3. Error handling is first-class: Result types, no unwrap abuse
4. Performance: Async everywhere, non-blocking UI
5. Mock mode for browser dev, real mode for Tauri

## Git
- Conventional commits
- Feature branches
- No commits to main without review

## Key Files
- `apps/web/src/lib/scanner/client.ts` - Scanner client (Tauri/mock detection)
- `apps/web/src/lib/stores/scan.svelte.ts` - Scan state with event listeners
- `packages/core/src/scanner/engine.rs` - Scanner engine with events
- `packages/core/src/fingerprint/vendor.rs` - MAC vendor lookup
