# Build Fixes Log

Issues encountered and resolved during build/testing:

## 1. PostCSS ES Module Error
**Error**: `module is not defined in ES module scope`
**Cause**: postcss.config.js used CommonJS `module.exports`
**Fix**: Changed to ES module `export default`

## 2. Missing tailwindcss-animate
**Error**: `Cannot find module 'tailwindcss-animate'`
**Fix**: `pnpm add -D tailwindcss-animate`

## 3. utils/index.ts Circular Export
**Error**: `formatDate` not found
**Cause**: File exported itself (`export * from './index'`)
**Fix**: Removed circular reference, added `formatDate` function

## 4. Svelte 5 Reactivity Warning
**Warning**: `This reference only captures the initial value of device`
**Cause**: `const Icon = deviceIcons[...]` not reactive
**Fix**: Changed to `$derived()`

## 5. Vitest Config Duplicate Key
**Error**: Duplicate "include" key in vite.config.ts
**Fix**: Merged into single include array with both paths

## Current Status
- ✅ Vite build: SUCCESS
- ✅ Tests: 7 passing (format + scan)
- ⚠️ devices.test.ts: 0 tests (needs investigation)

## Commands
```bash
# Install
pnpm install

# Dev
pnpm dev

# Build
pnpm vite build

# Test
pnpm test:run
```
