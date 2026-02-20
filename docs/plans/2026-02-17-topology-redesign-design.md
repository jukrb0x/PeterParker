# PeterParker UI Redesign: Network Topology Canvas

**Date:** 2026-02-17
**Status:** Approved
**Phase:** 1 of 2

## Summary

Redesign PeterParker's frontend with an interactive network topology visualization as the primary interface, replacing the current card grid view. This Phase 1 focuses on visual impact with a Jony Ive-inspired minimal aesthetic.

## Goals

1. Create a distinctive, memorable UI with topology as the "signature moment"
2. Maintain Jony Ive minimalism: clean surfaces, purposeful whitespace, subtle depth
3. Make network structure instantly visible at a glance
4. Preserve all existing functionality while improving visual hierarchy

## Scope

### In Scope (Phase 1)
- Interactive network topology canvas
- Device detail slide-in panel
- Refined header with search
- New scan configuration modal
- Visual design system refresh

### Deferred (Phase 2+)
- History & Analytics charts
- New Device Alerts system
- Device Tagging/Groups

## Design

### Overall Layout

```
┌─────────────────────────────────────────────────────────────┐
│  ○ PeterParker          [Search...]     [•••]  [Scan →]     │  <- Minimal header
├─────────────────────────────────────────────────────────────┤
│                                                             │
│              ┌─────────────────────────────────┐            │
│              │                                 │            │
│              │     NETWORK TOPOLOGY CANVAS     │            │
│              │                                 │            │
│              │   ○──○──○                       │            │
│              │   │     │                       │            │
│              │   ○     ○──○                    │            │
│              │         │                       │            │
│              │         ○                       │            │
│              │                                 │            │
│              └─────────────────────────────────┘            │
│                                                             │
│   24 devices  •  18 online  •  2 new today                  │  <- Floating stats
└─────────────────────────────────────────────────────────────┘
```

### Topology Canvas

**Node Types:**
| Type | Size | Position | Visual |
|------|------|----------|--------|
| Gateway/Router | 48px | Top-center | Largest, special icon |
| Regular Device | 36px | Orbit around gateway | Standard |
| Grouped (zoomed out) | Variable | Clustered | Shows count badge |
| New Device | 36px | Standard | Pulse animation + "NEW" |

**Node States:**
- Online: Cyan glow (`#00d4ff`)
- Offline: Dimmed gray (`#404040`), no glow
- Selected: Ring highlight

**Connection Lines:**
- 1px, subtle gray (`rgba(255,255,255,0.1)`)
- Connect devices to gateway
- Clean, no labels

**Layout Algorithm:**
- Force-directed layout
- Gateway pinned at top-center
- Devices positioned by "network distance" (response time)
- User drag to reposition (positions saved to localStorage)

**Interactions:**
| Action | Result |
|--------|--------|
| Hover node | Tooltip (IP, hostname, status) |
| Click node | Select, slide-in panel |
| Double-click | Navigate to device page |
| Drag node | Reposition |
| Drag canvas | Pan |
| Scroll/pinch | Zoom |
| Right-click | Context menu |

**Zoom Levels:**
- Far (100+ devices): Grouped clusters with counts
- Medium (20-100): Individual nodes, icons visible
- Near (<20): Larger nodes, labels visible

### Device Detail Panel

Slide-in panel from right when node is selected:

```
┌─────────────────────────────────┐
│ ← Back                    [⋮]  │
├─────────────────────────────────┤
│         [Device Icon]           │
│                                 │
│   192.168.1.1                   │
│   Gateway Router                │
│   ● Online                      │
│                                 │
├─────────────────────────────────┤
│   MAC    a0:f3:c1:xx:xx:xx     │
│   Vendor Ubiquiti Networks      │
│   Type   Router                 │
│   OS     EdgeOS 2.0             │
│   Ports  5 open                 │
│                                 │
├─────────────────────────────────┤
│   Open Ports                    │
│   ┌─────┬────────────────────┐  │
│   │ 22  │ SSH (OpenSSH 8.4)  │  │
│   │ 80  │ HTTP (nginx)       │  │
│   └─────┴────────────────────┘  │
│                                 │
├─────────────────────────────────┤
│   [Rescan]        [Full Page →] │
└─────────────────────────────────┘
```

**Specs:**
- Width: 320px fixed
- Background: `rgba(20,20,20,0.95)` with backdrop blur
- Animation: 300ms ease-out slide

### Header

Minimal single-row header:

```
┌───────────────────────────────────────────────────────────────┐
│ ○ PeterParker  │  [🔍 Search IP, hostname, MAC...]  │ ⋮ │ SCAN │
└───────────────────────────────────────────────────────────────┘
```

**Elements:**
- Logo: Small cyan circle + wordmark
- Search: Expands on focus
- Menu (⋮): Settings, Export, About
- Scan Button: Primary CTA

**Scan Button States:**
| State | Appearance |
|-------|------------|
| Idle | "SCAN" primary color |
| Scanning | Progress ring + "SCANNING..." |
| Complete | Checkmark → returns to "SCAN" |

### Scan Modal

Configuration modal when starting a scan:
- IP Range (auto-detected)
- Port Selection: Top 100 / Top 1000 / Custom
- Scan Type: Quick / Comprehensive

### Color System

```css
--pp-bg:            #0a0a0a    /* Background */
--pp-surface:       #141414    /* Cards/panels */
--pp-border:        #222222    /* Lines */
--pp-text:          #ffffff    /* Primary text */
--pp-text-muted:    #666666    /* Secondary text */
--pp-accent:        #00d4ff    /* Cyan - ONLY accent */
--pp-accent-glow:   rgba(0, 212, 255, 0.3)
--pp-success:       #22c55e    /* Online */
--pp-offline:       #404040    /* Offline */
```

### Typography

```css
--font-display:     "SF Pro Display", system-ui
--font-mono:        "SF Mono", "Fira Code"
```

Sizes:
- Headlines: 32px / 24px / 18px
- Body: 14px
- Mono: 13px
- Labels: 11px uppercase tracking-wide

## Technical Notes

### Topology Rendering Options

1. **D3.js + SVG** - Most common, good for ~500 nodes
2. **Cytoscape.js** - Purpose-built for network graphs
3. **Canvas 2D** - Better performance for 500+ nodes
4. **Svelte Canvas** - Svelte-native approach

Recommendation: Start with D3.js + SVG for simplicity, migrate to Canvas if performance issues.

### State Management

- Devices remain in existing `devicesStore`
- New `topologyStore` for:
  - Node positions (persisted to localStorage)
  - Canvas pan/zoom state
  - Selected node
  - Layout algorithm state

### File Structure

```
apps/web/src/lib/components/
├── topology/
│   ├── TopologyCanvas.svelte     # Main canvas component
│   ├── TopologyNode.svelte       # Individual node
│   ├── TopologyEdge.svelte       # Connection line
│   ├── TopologyTooltip.svelte    # Hover tooltip
│   └── index.ts
├── device/
│   ├── DeviceCard.svelte         # Keep for list view
│   ├── DevicePanel.svelte        # NEW: Slide-in panel
│   └── index.ts
├── scan/
│   ├── ScanProgress.svelte       # Keep
│   ├── ScanModal.svelte          # NEW: Config modal
│   └── index.ts
└── layout/
    ├── Header.svelte             # NEW: Extracted header
    └── index.ts
```

## Success Criteria

1. Topology displays all discovered devices in real-time during scan
2. Node interactions feel instant (hover, click, drag)
3. Layout is stable (doesn't shift unexpectedly)
4. Works on both browser (mock) and Tauri modes
5. Zero console errors
6. Feels "polished" - animations smooth, spacing consistent

## Risks

1. **Performance** - Force-directed layout can be slow with 100+ nodes
   - Mitigation: Use WebWorker for layout, Canvas for rendering
2. **Mobile** - Touch interactions need careful implementation
   - Mitigation: Responsive fallback to card grid on mobile
3. **Complexity** - More code to maintain
   - Mitigation: Keep components small, well-tested

## Next Steps

1. Create implementation plan with detailed tasks
2. Set up topology component skeleton
3. Implement D3.js force-directed layout
4. Build node/edge rendering
5. Add interactions (hover, click, pan, zoom)
6. Build slide-in panel
7. Refine header
8. Add scan modal
9. Polish animations
10. Test with mock data, then Tauri
