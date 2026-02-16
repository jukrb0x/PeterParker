# Tech Stack Decisions

## Why Rust (Not Go)?

### 1. Tauri Native Integration
Tauri is Rust-based. Using Go would require:
- CGO bindings (complex, brittle)
- Separate process communication (IPC overhead)
- Double the binary size (defeats Tauri's purpose)

### 2. Performance Characteristics
| Aspect | Rust | Go |
|--------|------|-----|
| Memory | Zero-cost, no GC | GC pauses |
| Async | Tokio (predictable) | Goroutines (good, but...) |
| Binary | ~5MB | ~15MB+ with CGO |
| Raw sockets | Native support | Possible, complex |

### 3. Network Scanning Requirements
Network scanners need:
- **Precise timing**: SYN scan timeouts in 1-10ms range
- **High concurrency**: 1000+ parallel connections
- **Raw socket access**: ARP requires link-layer control
- **No GC pauses**: Stalls during scan = missed packets

Rust's ownership model + Tokio's async runtime excel here.

### 4. When Go Would Be Better
- API server (fast build, good stdlib)
- CLI tools without GUI (single binary)
- Team already knows Go

### 5. When Rust Wins
- System tools (network, security)
- GUI apps with Tauri
- Performance-critical paths
- Cross-platform consistency

## Package Manager: pnpm

```bash
# Install dependencies
pnpm install

# Development
pnpm tauri:dev

# Build
pnpm tauri:build

# Test
pnpm test
```

### Why pnpm?
- Disk efficient (hard links)
- Fast (parallel downloads)
- Strict (prevents phantom deps)
- Works with Tauri out of the box

## Summary

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Desktop | Tauri v2 | Rust-native, small binary |
| Frontend | SvelteKit | Compiler-based, minimal runtime |
| State | Svelte 5 runes | Reactive, type-safe |
| UI | shadcn-svelte | Accessible, customizable |
| Backend | Rust | Tauri integration, performance |
| Async | Tokio | Industry standard for Rust |
| Testing | vitest + cargo test | Native for each layer |
