# PeterParker - LAN Scanner App

## Mission
Cross-platform LAN device scanner with detailed device fingerprinting.

## Tech Stack
- **Desktop**: Tauri v2 (Rust + WebView)
- **UI**: TypeScript + SvelteKit + shadcn-svelte + Tailwind CSS
- **Backend**: Rust (Tauri commands + standalone modules)
- **Testing**: vitest (frontend), cargo test (Rust)

## Design Philosophy
- **Jony Ive minimalism**: Clean surfaces, purposeful whitespace, subtle depth
- **Radical efficiency**: No wasted motion, every pixel serves function
- **Information density**: Rich data, elegant presentation

## Architecture Principles
- Separation of concerns: Scanner engine ↔ State management ↔ UI
- Reactive data flow: Svelte stores → derived state → component bindings
- Extensible scanner modules: Protocol-based plugin system
- Type safety: End-to-end TypeScript + Rust strict types

## Project Structure
```
/src-tauri/        Rust backend, scanner engine
/src/             SvelteKit frontend
  /lib/
    /components/  shadcn-svelte components
    /stores/      Reactive state
    /scanner/     Scanner client API
    /types/       TypeScript definitions
  /routes/        SvelteKit routes
/tests/           vitest tests
```

## Development Rules
1. No placeholder code. Every function has purpose.
2. Tests required: unit for logic, integration for scanner API
3. Error handling is first-class: Result types, no unwrap abuse
4. Performance: Async everywhere, non-blocking UI

## Git
- Conventional commits
- Feature branches
- No commits to main without review
