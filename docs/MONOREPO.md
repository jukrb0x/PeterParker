# Monorepo Structure

PeterParker is organized as a pnpm workspace monorepo with Turbo.

## Structure

```
peterparker/
├── apps/
│   ├── web/              # SvelteKit frontend
│   └── tauri/            # Tauri desktop app
├── packages/
│   └── core/             # Rust core library
├── turbo.json            # Turbo pipeline config
├── pnpm-workspace.yaml   # pnpm workspace config
└── package.json          # Root package.json
```

## Apps

### @peterparker/web
Location: `apps/web/`
- SvelteKit frontend
- TypeScript + Svelte 5
- shadcn-svelte UI components
- Vitest tests

### @peterparker/tauri
Location: `apps/tauri/`
- Tauri v2 desktop app
- Depends on `peterparker-core`
- Shell plugin for ping commands

## Packages

### peterparker-core
Location: `packages/core/`
- Rust library
- Network scanning logic
- Device fingerprinting
- Models and types

## Commands

```bash
# Install dependencies
pnpm install

# Run web dev
pnpm --filter @peterparker/web dev

# Run Tauri dev
pnpm tauri:dev

# Build all
pnpm build

# Test all
pnpm test

# Test specific package
pnpm --filter @peterparker/web test:run
```
