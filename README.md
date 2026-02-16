# PeterParker - Quick Start

## Development Commands

```bash
# Install dependencies
pnpm install

# Run web only (SvelteKit dev server)
pnpm dev:web

# Run Tauri desktop app
pnpm dev:tauri

# Build everything
pnpm build

# Run all tests
pnpm test:run
```

## Project Structure

```
peterparker/
├── apps/
│   ├── web/           # SvelteKit frontend (@peterparker/web)
│   └── tauri/         # Tauri desktop app
├── packages/
│   └── core/          # Rust core library (peterparker-core)
├── package.json       # Root package.json with scripts
├── turbo.json         # Turbo pipeline config
└── pnpm-workspace.yaml
```

## Tech Stack

- **Frontend**: SvelteKit 5 + TypeScript + Tailwind CSS + shadcn-svelte
- **Desktop**: Tauri v2 (Rust)
- **Core**: Rust library for network scanning
- **Package Manager**: pnpm with workspaces
- **Build Orchestration**: Turbo

## Requirements

- Node.js 18+
- Rust 1.75+
- pnpm 9+

## Test Status

- ✅ Web build: PASS
- ✅ Rust core tests: 4 passing
- ⚠️ Web tests: needs vitest fix (svelte testing library compatibility)
