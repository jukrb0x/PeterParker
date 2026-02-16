# Monorepo Structure

PeterParker is organized as a pnpm workspace with Rust workspace integration.

## Structure

```
peterparker/
├── apps/
│   ├── web/              # SvelteKit frontend (@peterparker/web)
│   └── tauri/            # Tauri desktop app
├── packages/
│   └── core/             # Rust core library (peterparker-core)
├── Cargo.toml            # Rust workspace
├── turbo.json            # Turbo pipeline config
├── pnpm-workspace.yaml   # pnpm workspace config
└── package.json          # Root npm scripts
```

## Apps

### @peterparker/web
Location: `apps/web/`
- SvelteKit frontend with Svelte 5
- TypeScript + Tailwind CSS
- shadcn-svelte UI components
- Vitest tests
- Can run standalone with mock scanner data

### Tauri
Location: `apps/tauri/`
- Tauri v2 desktop app
- Embeds `peterparker-core` for real scanning
- Real-time events via Tauri IPC

## Packages

### peterparker-core
Location: `packages/core/`
- Rust library with network scanning logic
- Scanner engine with ARP/TCP/HTTP modules
- Vendor lookup (44,000+ MAC OUIs)
- Device fingerprinting

## Commands

### Development
```bash
# Web only (mock data)
pnpm dev
pnpm dev:web

# Desktop (real scanning)
pnpm dev:tauri
```

### Building
```bash
# Web
pnpm build:web

# Desktop (all platforms)
pnpm build:tauri

# Platform-specific
pnpm build:tauri:mac     # macOS Universal
pnpm build:tauri:win     # Windows x64
pnpm build:tauri:linux   # Linux x64
```

### Testing
```bash
# All tests
pnpm test

# Specific packages
pnpm test:core        # Rust tests only
pnpm test:web         # Web tests only
```

### Maintenance
```bash
pnpm check            # Type check all
pnpm lint             # Lint all
pnpm format           # Format all
pnpm clean            # Clean all builds
```

## Workspaces

### pnpm Workspace
Managed by `pnpm-workspace.yaml`:
```yaml
packages:
  - 'apps/*'
  - 'packages/*'
```

### Rust Workspace
Managed by root `Cargo.toml`:
```toml
[workspace]
members = [
    "packages/core",
    "apps/tauri/src-tauri",
]
```

## Development Modes

1. **Web Mode** (`pnpm dev`): Browser with mock scanner data
2. **Tauri Mode** (`pnpm dev:tauri`): Desktop with real network scanning

Both share the same UI components and state management.
