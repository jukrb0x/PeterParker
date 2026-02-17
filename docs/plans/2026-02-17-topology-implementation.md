# Topology Canvas Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the card grid with an interactive network topology visualization as the primary interface.

**Architecture:** D3.js force-directed layout rendered via SVG. Devices become nodes connected to a gateway. Node positions persist to localStorage. Clicking a node opens a slide-in detail panel.

**Tech Stack:** Svelte 5, D3.js, SVG, Tailwind CSS, lucide-svelte

---

## Task 1: Install D3.js Dependency

**Files:**
- Modify: `apps/web/package.json`

**Step 1: Install D3.js**

Run:
```bash
cd apps/web && pnpm add d3 && pnpm add -D @types/d3
```

Expected: `d3` and `@types/d3` added to dependencies/devDependencies

**Step 2: Verify installation**

Run:
```bash
cd apps/web && pnpm ls d3
```

Expected: Shows `d3 x.x.x`

---

## Task 2: Create Topology Types

**Files:**
- Create: `apps/web/src/lib/topology/types.ts`

**Step 1: Create types file**

```typescript
// apps/web/src/lib/topology/types.ts
import type { Device } from '$lib/scanner/types';

export interface TopologyNode {
	id: string;
	device: Device;
	x: number;
	y: number;
	vx: number;
	vy: number;
	fx: number | null; // fixed x position
	fy: number | null; // fixed y position
	isGateway: boolean;
	isNew: boolean;
}

export interface TopologyEdge {
	id: string;
	source: string; // node id
	target: string; // node id (gateway)
}

export interface TopologyState {
	nodes: TopologyNode[];
	edges: TopologyEdge[];
	selectedNodeId: string | null;
	hoveredNodeId: string | null;
	pan: { x: number; y: number };
	zoom: number;
	isDragging: boolean;
}

export interface NodePosition {
	id: string;
	x: number;
	y: number;
}

export const STORAGE_KEY = 'peterparker-topology-positions';

// Node sizes
export const GATEWAY_NODE_SIZE = 48;
export const DEVICE_NODE_SIZE = 36;

// Colors (matching design)
export const COLORS = {
	online: '#00d4ff',
	onlineGlow: 'rgba(0, 212, 255, 0.3)',
	offline: '#404040',
	selected: '#00d4ff',
	edge: 'rgba(255, 255, 255, 0.1)'
};
```

**Step 2: Verify types compile**

Run:
```bash
cd apps/web && pnpm check
```

Expected: No errors

---

## Task 3: Create Topology Store

**Files:**
- Create: `apps/web/src/lib/topology/store.svelte.ts`

**Step 1: Create the store**

```svelte
// apps/web/src/lib/topology/store.svelte.ts
import type { Device } from '$lib/scanner/types';
import type { TopologyNode, TopologyEdge, TopologyState, NodePosition } from './types';
import { STORAGE_KEY, COLORS, GATEWAY_NODE_SIZE, DEVICE_NODE_SIZE } from './types';

class TopologyStore {
	// State
	state = $state<TopologyState>({
		nodes: [],
		edges: [],
		selectedNodeId: null,
		hoveredNodeId: null,
		pan: { x: 0, y: 0 },
		zoom: 1,
		isDragging: false
	});

	// Persisted positions
	savedPositions = $state<Map<string, { x: number; y: number }>>(new Map());

	// Derived
	selectedNode = $derived(
		this.state.selectedNodeId
			? this.state.nodes.find(n => n.id === this.state.selectedNodeId)
			: null
	);

	hoveredNode = $derived(
		this.state.hoveredNodeId
			? this.state.nodes.find(n => n.id === this.state.hoveredNodeId)
			: null
	);

	gatewayNode = $derived(this.state.nodes.find(n => n.isGateway));

	constructor() {
		this.loadPositions();
	}

	// Load saved positions from localStorage
	private loadPositions(): void {
		if (typeof window === 'undefined') return;
		try {
			const saved = localStorage.getItem(STORAGE_KEY);
			if (saved) {
				const positions: NodePosition[] = JSON.parse(saved);
				this.savedPositions = new Map(positions.map(p => [p.id, { x: p.x, y: p.y }]));
			}
		} catch (e) {
			console.warn('Failed to load topology positions:', e);
		}
	}

	// Save positions to localStorage
	savePositions(): void {
		if (typeof window === 'undefined') return;
		try {
			const positions: NodePosition[] = this.state.nodes.map(n => ({
				id: n.id,
				x: n.x,
				y: n.y
			}));
			localStorage.setItem(STORAGE_KEY, JSON.stringify(positions));
		} catch (e) {
			console.warn('Failed to save topology positions:', e);
		}
	}

	// Convert devices to topology nodes
	setDevices(devices: Device[], canvasWidth: number, canvasHeight: number): void {
		const existingNodes = new Map(this.state.nodes.map(n => [n.id, n]));

		// Find gateway (first router, or first device if no router)
		const gateway = devices.find(d => d.deviceType === 'router') || devices[0];
		const gatewayId = gateway?.id;

		this.state.nodes = devices.map((device, index) => {
			const existing = existingNodes.get(device.id);
			const saved = this.savedPositions.get(device.id);
			const isGateway = device.id === gatewayId;

			// Check if device is "new" (first seen within last 24 hours)
			const firstSeen = new Date(device.firstSeen);
			const isNew = Date.now() - firstSeen.getTime() < 24 * 60 * 60 * 1000;

			return {
				id: device.id,
				device,
				// Use saved position, then existing position, then calculate new
				x: saved?.x ?? existing?.x ?? this.calculateInitialX(index, devices.length, canvasWidth),
				y: saved?.y ?? existing?.y ?? this.calculateInitialY(index, devices.length, canvasHeight, isGateway),
				vx: 0,
				vy: 0,
				// Gateway is fixed at top-center
				fx: isGateway ? canvasWidth / 2 : null,
				fy: isGateway ? 80 : null,
				isGateway,
				isNew
			};
		});

		// Create edges (all connect to gateway)
		this.state.edges = devices
			.filter(d => d.id !== gatewayId)
			.map(d => ({
				id: `${d.id}-${gatewayId}`,
				source: d.id,
				target: gatewayId
			}));
	}

	private calculateInitialX(index: number, total: number, width: number): number {
		// Spread nodes in a circle around gateway
		const angle = (index / total) * 2 * Math.PI - Math.PI / 2;
		const radius = Math.min(width, 400) * 0.35;
		return width / 2 + Math.cos(angle) * radius;
	}

	private calculateInitialY(index: number, total: number, height: number, isGateway: boolean): number {
		if (isGateway) return 80;
		const angle = (index / total) * 2 * Math.PI - Math.PI / 2;
		const radius = Math.min(height, 400) * 0.35;
		return height / 3 + Math.sin(angle) * radius;
	}

	// Selection
	selectNode(id: string | null): void {
		this.state.selectedNodeId = id;
	}

	hoverNode(id: string | null): void {
		this.state.hoveredNodeId = id;
	}

	// Pan and zoom
	setPan(x: number, y: number): void {
		this.state.pan = { x, y };
	}

	setZoom(zoom: number): void {
		this.state.zoom = Math.max(0.1, Math.min(3, zoom));
	}

	// Update node position after drag
	updateNodePosition(id: string, x: number, y: number): void {
		const node = this.state.nodes.find(n => n.id === id);
		if (node && !node.isGateway) {
			node.x = x;
			node.y = y;
			this.savedPositions.set(id, { x, y });
		}
	}

	setDragging(isDragging: boolean): void {
		this.state.isDragging = isDragging;
	}

	// Get node color based on state
	getNodeColor(node: TopologyNode): string {
		if (!node.device.isOnline) return COLORS.offline;
		return COLORS.online;
	}

	getNodeGlow(node: TopologyNode): string {
		if (!node.device.isOnline) return 'transparent';
		if (this.state.selectedNodeId === node.id) return COLORS.onlineGlow;
		return 'transparent';
	}
}

export const topologyStore = new TopologyStore();
```

**Step 2: Verify store compiles**

Run:
```bash
cd apps/web && pnpm check
```

Expected: No errors

---

## Task 4: Create Topology Index

**Files:**
- Create: `apps/web/src/lib/topology/index.ts`

**Step 1: Create index**

```typescript
// apps/web/src/lib/topology/index.ts
export * from './types';
export { topologyStore } from './store.svelte';
```

---

## Task 5: Create TopologyCanvas Component

**Files:**
- Create: `apps/web/src/lib/components/topology/TopologyCanvas.svelte`

**Step 1: Create the canvas component**

```svelte
<script lang="ts">
	import { onMount } from 'svelte';
	import * as d3 from 'd3';
	import type { Device } from '$lib/scanner/types';
	import { topologyStore } from '$lib/topology/store.svelte';
	import { GATEWAY_NODE_SIZE, DEVICE_NODE_SIZE } from '$lib/topology/types';
	import TopologyNode from './TopologyNode.svelte';
	import TopologyEdge from './TopologyEdge.svelte';
	import TopologyTooltip from './TopologyTooltip.svelte';

	interface Props {
		devices: Device[];
		onNodeClick?: (deviceId: string) => void;
		onNodeDoubleClick?: (deviceId: string) => void;
	}

	let { devices, onNodeClick, onNodeDoubleClick }: Props = $props();

	let container: HTMLDivElement;
	let width = $state(800);
	let height = $state(600);

	// D3 simulation
	let simulation: d3.Simulation<any, undefined> | null = null;

	// Watch for device changes
	$effect(() => {
		if (devices.length > 0 && width > 0 && height > 0) {
			topologyStore.setDevices(devices, width, height);
			initSimulation();
		}
	});

	onMount(() => {
		updateDimensions();
		window.addEventListener('resize', updateDimensions);

		return () => {
			window.removeEventListener('resize', updateDimensions);
			if (simulation) simulation.stop();
		};
	});

	function updateDimensions() {
		if (container) {
			width = container.clientWidth;
			height = container.clientHeight;
		}
	}

	function initSimulation() {
		if (simulation) simulation.stop();

		const nodes = topologyStore.state.nodes.map(n => ({ ...n }));

		simulation = d3.forceSimulation(nodes)
			.force('link', d3.forceLink()
				.id((d: any) => d.id)
				.distance(120)
				.strength(0.5)
			)
			.force('charge', d3.forceManyBody().strength(-200))
			.force('center', d3.forceCenter(width / 2, height / 2))
			.force('collision', d3.forceCollide().radius(GATEWAY_NODE_SIZE))
			.alphaDecay(0.02)
			.on('tick', () => {
				// Update store positions from simulation
				nodes.forEach((simNode: any) => {
					const storeNode = topologyStore.state.nodes.find(n => n.id === simNode.id);
					if (storeNode && !storeNode.isGateway) {
						storeNode.x = simNode.x;
						storeNode.y = simNode.y;
					}
				});
			});

		// Set link data
		(simulation as any).force('link')?.links(topologyStore.state.edges);
	}

	function handleNodeClick(id: string) {
		topologyStore.selectNode(id);
		onNodeClick?.(id);
	}

	function handleNodeDoubleClick(id: string) {
		onNodeDoubleClick?.(id);
	}

	function handleNodeDrag(id: string, x: number, y: number) {
		topologyStore.updateNodePosition(id, x, y);
		if (simulation) {
			const node = simulation.nodes().find((n: any) => n.id === id);
			if (node) {
				(node as any).fx = x;
				(node as any).fy = y;
				simulation.alpha(0.3).restart();
			}
		}
	}

	function handleWheel(e: WheelEvent) {
		e.preventDefault();
		const delta = e.deltaY > 0 ? 0.9 : 1.1;
		topologyStore.setZoom(topologyStore.state.zoom * delta);
	}
</script>

<div
	bind:this={container}
	class="relative w-full h-full overflow-hidden"
	onwheel={handleWheel}
>
	<svg
		width={width}
		height={height}
		class="absolute inset-0"
	>
		<!-- Transform group for pan/zoom -->
		<g transform="translate({topologyStore.state.pan.x}, {topologyStore.state.pan.y}) scale({topologyStore.state.zoom})">
			<!-- Edges -->
			{#each topologyStore.state.edges as edge (edge.id)}
				<TopologyEdge {edge} />
			{/each}

			<!-- Nodes -->
			{#each topologyStore.state.nodes as node (node.id)}
				<TopologyNode
					{node}
					onclick={() => handleNodeClick(node.id)}
					ondblclick={() => handleNodeDoubleClick(node.id)}
					onmouseenter={() => topologyStore.hoverNode(node.id)}
					onmouseleave={() => topologyStore.hoverNode(null)}
					ondrag={(x, y) => handleNodeDrag(node.id, x, y)}
				/>
			{/each}
		</g>
	</svg>

	<!-- Tooltip -->
	{#if topologyStore.hoveredNode}
		<TopologyTooltip node={topologyStore.hoveredNode!} />
	{/if}
</div>
```

---

## Task 6: Create TopologyNode Component

**Files:**
- Create: `apps/web/src/lib/components/topology/TopologyNode.svelte`

**Step 1: Create the node component**

```svelte
<script lang="ts">
	import type { TopologyNode } from '$lib/topology/types';
	import { topologyStore } from '$lib/topology/store.svelte';
	import { GATEWAY_NODE_SIZE, DEVICE_NODE_SIZE, COLORS } from '$lib/topology/types';
	import { Router, Laptop, Smartphone, Printer, HardDrive, Tv, Gamepad2, HelpCircle, Wifi, Camera, Server, Monitor } from 'lucide-svelte';

	interface Props {
		node: TopologyNode;
		onclick?: () => void;
		ondblclick?: () => void;
		onmouseenter?: () => void;
		onmouseleave?: () => void;
		ondrag?: (x: number, y: number) => void;
	}

	let { node, onclick, ondblclick, onmouseenter, onmouseleave, ondrag }: Props = $props();

	const size = $derived(node.isGateway ? GATEWAY_NODE_SIZE : DEVICE_NODE_SIZE);
	const isSelected = $derived(topologyStore.state.selectedNodeId === node.id);
	const color = $derived(topologyStore.getNodeColor(node));

	// Drag state
	let isDragging = $state(false);
	let dragStart = $state({ x: 0, y: 0 });

	function handleMouseDown(e: MouseEvent) {
		if (node.isGateway) return;
		isDragging = true;
		dragStart = { x: e.clientX, y: e.clientY };
		topologyStore.setDragging(true);
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isDragging) return;
		const dx = e.clientX - dragStart.x;
		const dy = e.clientY - dragStart.y;
		ondrag?.(node.x + dx / topologyStore.state.zoom, node.y + dy / topologyStore.state.zoom);
		dragStart = { x: e.clientX, y: e.clientY };
	}

	function handleMouseUp() {
		if (isDragging) {
			isDragging = false;
			topologyStore.setDragging(false);
			topologyStore.savePositions();
		}
	}
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={handleMouseUp} />

<g
	class="topology-node"
	class:dragging={isDragging}
	class:selected={isSelected}
	class:gateway={node.isGateway}
	class:new={node.isNew}
	class:offline={!node.device.isOnline}
	transform="translate({node.x}, {node.y})"
	role="button"
	tabindex="0"
	{onclick}
	{ondblclick}
	{onmouseenter}
	{onmouseleave}
	onmousedown={handleMouseDown}
>
	<!-- Glow effect for online nodes -->
	{#if node.device.isOnline}
		<circle
			r={size / 2 + 8}
			fill={COLORS.onlineGlow}
			class="glow"
		/>
	{/if}

	<!-- Selection ring -->
	{#if isSelected}
		<circle
			r={size / 2 + 4}
			fill="none"
			stroke={COLORS.selected}
			stroke-width="2"
		/>
	{/if}

	<!-- Main circle -->
	<circle
		r={size / 2}
		fill="#141414"
		stroke={color}
		stroke-width="2"
		class="node-circle"
	/>

	<!-- Device type icon (simplified - just a circle for now, icon rendered separately) -->

	<!-- New badge -->
	{#if node.isNew}
		<g transform="translate({size / 2 - 4}, {-size / 2 + 4})">
			<circle r="8" fill="#22c55e" />
			<text
				x="0"
				y="0"
				text-anchor="middle"
				dominant-baseline="central"
				fill="white"
				font-size="8"
				font-weight="bold"
			>
				N
			</text>
		</g>
	{/if}
</g>

<style>
	.topology-node {
		cursor: pointer;
		transition: transform 0.1s ease;
	}

	.topology-node:hover .node-circle {
		fill: #1a1a1a;
	}

	.topology-node.dragging {
		cursor: grabbing;
	}

	.topology-node.gateway {
		cursor: default;
	}

	.glow {
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 0.3; }
		50% { opacity: 0.6; }
	}

	.topology-node.offline .glow {
		display: none;
	}
</style>
```

---

## Task 7: Create TopologyEdge Component

**Files:**
- Create: `apps/web/src/lib/components/topology/TopologyEdge.svelte`

**Step 1: Create the edge component**

```svelte
<script lang="ts">
	import type { TopologyEdge } from '$lib/topology/types';
	import { topologyStore } from '$lib/topology/store.svelte';
	import { COLORS } from '$lib/topology/types';

	interface Props {
		edge: TopologyEdge;
	}

	let { edge }: Props = $props();

	// Find node positions
	const sourceNode = $derived(topologyStore.state.nodes.find(n => n.id === edge.source));
	const targetNode = $derived(topologyStore.state.nodes.find(n => n.id === edge.target));

	// Highlight if either end is selected/hovered
	const isHighlighted = $derived(
		topologyStore.state.selectedNodeId === edge.source ||
		topologyStore.state.selectedNodeId === edge.target ||
		topologyStore.state.hoveredNodeId === edge.source ||
		topologyStore.state.hoveredNodeId === edge.target
	);
</script>

{#if sourceNode && targetNode}
	<line
		x1={sourceNode.x}
		y1={sourceNode.y}
		x2={targetNode.x}
		y2={targetNode.y}
		stroke={isHighlighted ? COLORS.online : COLORS.edge}
		stroke-width={isHighlighted ? 2 : 1}
		class="topology-edge"
		class:highlighted={isHighlighted}
	/>
{/if}

<style>
	.topology-edge {
		transition: stroke 0.2s ease, stroke-width 0.2s ease;
		pointer-events: none;
	}

	.topology-edge.highlighted {
		stroke-opacity: 0.5;
	}
</style>
```

---

## Task 8: Create TopologyTooltip Component

**Files:**
- Create: `apps/web/src/lib/components/topology/TopologyTooltip.svelte`

**Step 1: Create the tooltip component**

```svelte
<script lang="ts">
	import type { TopologyNode } from '$lib/topology/types';

	interface Props {
		node: TopologyNode;
	}

	let { node }: Props = $props();
</script>

<div class="tooltip">
	<div class="tooltip-header">
		<span class="ip">{node.device.ip}</span>
		<span class="status" class:online={node.device.isOnline} class:offline={!node.device.isOnline}>
			{node.device.isOnline ? 'Online' : 'Offline'}
		</span>
	</div>
	{#if node.device.hostname}
		<div class="tooltip-hostname">{node.device.hostname}</div>
	{/if}
	{#if node.device.vendor}
		<div class="tooltip-vendor">{node.device.vendor}</div>
	{/if}
</div>

<style>
	.tooltip {
		position: absolute;
		top: 20px;
		left: 20px;
		background: rgba(20, 20, 20, 0.95);
		backdrop-filter: blur(8px);
		border: 1px solid #222;
		border-radius: 8px;
		padding: 12px 16px;
		font-size: 14px;
		pointer-events: none;
		z-index: 100;
		min-width: 160px;
	}

	.tooltip-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 12px;
	}

	.ip {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 13px;
		color: #fff;
	}

	.status {
		font-size: 11px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.status.online {
		color: #00d4ff;
	}

	.status.offline {
		color: #666;
	}

	.tooltip-hostname {
		margin-top: 4px;
		color: #fff;
		font-weight: 500;
	}

	.tooltip-vendor {
		margin-top: 2px;
		color: #666;
		font-size: 12px;
	}
</style>
```

---

## Task 9: Create Topology Components Index

**Files:**
- Create: `apps/web/src/lib/components/topology/index.ts`

**Step 1: Create index**

```typescript
// apps/web/src/lib/components/topology/index.ts
export { default as TopologyCanvas } from './TopologyCanvas.svelte';
export { default as TopologyNode } from './TopologyNode.svelte';
export { default as TopologyEdge } from './TopologyEdge.svelte';
export { default as TopologyTooltip } from './TopologyTooltip.svelte';
```

---

## Task 10: Create DevicePanel Component

**Files:**
- Create: `apps/web/src/lib/components/device/DevicePanel.svelte`

**Step 1: Create the slide-in panel**

```svelte
<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Device } from '$lib/scanner/types';
	import { topologyStore } from '$lib/topology/store.svelte';
	import { Button } from '$lib/components/ui/button';
	import { formatDate } from '$lib/utils';
	import { X, ExternalLink, RotateCw, Router, Laptop, Smartphone, Printer, HardDrive, Tv, Gamepad2, HelpCircle, Wifi, Camera, Server, Monitor } from 'lucide-svelte';

	interface Props {
		device: Device | null;
		onClose: () => void;
		onRescan?: () => void;
	}

	let { device, onClose, onRescan }: Props = $props();

	const deviceIconMap: Record<string, typeof Laptop> = {
		desktop: Monitor,
		laptop: Laptop,
		mobile: Smartphone,
		tablet: Smartphone,
		router: Router,
		switch: Router,
		printer: Printer,
		nas: HardDrive,
		tv: Tv,
		game_console: Gamepad2,
		gameconsole: Gamepad2,
		iot: Wifi,
		camera: Camera,
		server: Server,
		unknown: HelpCircle
	};

	const Icon = $derived(device ? (deviceIconMap[device.deviceType] || HelpCircle) : HelpCircle);

	function viewFullPage() {
		if (device) {
			goto(`/device/${encodeURIComponent(device.ip)}`);
		}
	}
</script>

{#if device}
	<div class="panel-overlay" onclick={onClose}></div>

	<aside class="device-panel" class:open={device}>
		<header class="panel-header">
			<button class="close-btn" onclick={onClose}>
				<X class="h-5 w-5" />
			</button>
			<div class="header-actions">
				<Button variant="ghost" size="icon" onclick={onRescan}>
					<RotateCw class="h-4 w-4" />
				</Button>
			</div>
		</header>

		<div class="panel-content">
			<!-- Device icon and name -->
			<div class="device-header">
				<div class="device-icon">
					<Icon class="h-8 w-8" />
				</div>
				<div class="device-info">
					<h2 class="device-name">{device.hostname || device.vendor || 'Unknown Device'}</h2>
					<p class="device-ip">{device.ip}</p>
					<div class="device-status">
						<span class="status-dot" class:online={device.isOnline}></span>
						<span>{device.isOnline ? 'Online' : 'Offline'}</span>
					</div>
				</div>
			</div>

			<!-- Device details -->
			<div class="detail-grid">
				{#if device.mac}
					<div class="detail-item">
						<span class="detail-label">MAC</span>
						<span class="detail-value mono">{device.mac}</span>
					</div>
				{/if}
				{#if device.vendor}
					<div class="detail-item">
						<span class="detail-label">Vendor</span>
						<span class="detail-value">{device.vendor}</span>
					</div>
				{/if}
				<div class="detail-item">
					<span class="detail-label">Type</span>
					<span class="detail-value capitalize">{device.deviceType.replace('_', ' ')}</span>
				</div>
				{#if device.os?.name}
					<div class="detail-item">
						<span class="detail-label">OS</span>
						<span class="detail-value">{device.os.name}</span>
					</div>
				{/if}
				<div class="detail-item">
					<span class="detail-label">Ports</span>
					<span class="detail-value">{device.ports.length} open</span>
				</div>
			</div>

			<!-- Open ports -->
			{#if device.ports.length > 0}
				<div class="ports-section">
					<h3 class="section-title">Open Ports</h3>
					<div class="ports-list">
						{#each device.ports.slice(0, 5) as port}
							<div class="port-item">
								<span class="port-number mono">{port.number}</span>
								<span class="port-service">{port.service?.name || port.protocol}</span>
							</div>
						{/each}
						{#if device.ports.length > 5}
							<div class="more-ports">+{device.ports.length - 5} more</div>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Actions -->
			<div class="panel-actions">
				<Button variant="outline" onclick={onRescan}>
					<RotateCw class="mr-2 h-4 w-4" />
					Rescan
				</Button>
				<Button onclick={viewFullPage}>
					<ExternalLink class="mr-2 h-4 w-4" />
					Full Page
				</Button>
			</div>
		</div>
	</aside>
{/if}

<style>
	.panel-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		z-index: 40;
		opacity: 0;
		animation: fadeIn 0.3s ease forwards;
	}

	@keyframes fadeIn {
		to { opacity: 1; }
	}

	.device-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: 320px;
		height: 100vh;
		background: rgba(20, 20, 20, 0.95);
		backdrop-filter: blur(12px);
		border-left: 1px solid #222;
		z-index: 50;
		transform: translateX(100%);
		transition: transform 0.3s ease-out;
		display: flex;
		flex-direction: column;
	}

	.device-panel.open {
		transform: translateX(0);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px;
		border-bottom: 1px solid #222;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 6px;
		background: transparent;
		border: none;
		color: #666;
		cursor: pointer;
		transition: background 0.2s, color 0.2s;
	}

	.close-btn:hover {
		background: #222;
		color: #fff;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
	}

	.device-header {
		display: flex;
		align-items: center;
		gap: 16px;
		margin-bottom: 24px;
	}

	.device-icon {
		width: 56px;
		height: 56px;
		border-radius: 12px;
		background: #1a1a1a;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #00d4ff;
	}

	.device-info {
		flex: 1;
	}

	.device-name {
		font-size: 18px;
		font-weight: 600;
		color: #fff;
		margin: 0;
	}

	.device-ip {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 13px;
		color: #666;
		margin: 4px 0;
	}

	.device-status {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		color: #666;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #404040;
	}

	.status-dot.online {
		background: #00d4ff;
		box-shadow: 0 0 8px rgba(0, 212, 255, 0.5);
	}

	.detail-grid {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-bottom: 24px;
	}

	.detail-item {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
		border-bottom: 1px solid #1a1a1a;
	}

	.detail-label {
		font-size: 11px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #666;
	}

	.detail-value {
		font-size: 14px;
		color: #fff;
	}

	.detail-value.mono {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 13px;
	}

	.detail-value.capitalize {
		text-transform: capitalize;
	}

	.ports-section {
		margin-bottom: 24px;
	}

	.section-title {
		font-size: 11px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #666;
		margin-bottom: 12px;
	}

	.ports-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.port-item {
		display: flex;
		justify-content: space-between;
		padding: 8px 12px;
		background: #1a1a1a;
		border-radius: 6px;
	}

	.port-number {
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 13px;
		color: #00d4ff;
	}

	.port-service {
		font-size: 13px;
		color: #666;
	}

	.more-ports {
		font-size: 12px;
		color: #666;
		text-align: center;
		padding: 8px;
	}

	.panel-actions {
		display: flex;
		gap: 8px;
		padding-top: 16px;
		border-top: 1px solid #222;
	}

	.panel-actions :global(button) {
		flex: 1;
	}
</style>
```

---

## Task 11: Update Device Index

**Files:**
- Modify: `apps/web/src/lib/components/device/index.ts`

**Step 1: Add DevicePanel export**

```typescript
// apps/web/src/lib/components/device/index.ts
export { default as DeviceCard } from './DeviceCard.svelte';
export { default as DevicePanel } from './DevicePanel.svelte';
```

---

## Task 12: Create Header Component

**Files:**
- Create: `apps/web/src/lib/components/layout/Header.svelte`

**Step 1: Create the header component**

```svelte
<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Search, Settings, Wifi, Loader } from 'lucide-svelte';

	interface Props {
		isScanning?: boolean;
		scanProgress?: number;
		onScan?: () => void;
		searchQuery?: string;
		onSearchChange?: (query: string) => void;
	}

	let { isScanning = false, scanProgress = 0, onScan, searchQuery = '', onSearchChange }: Props = $props();

	let searchFocused = $state(false);
</script>

<header class="header">
	<div class="header-left">
		<div class="logo">
			<div class="logo-dot"></div>
			<span class="logo-text">PeterParker</span>
		</div>
	</div>

	<div class="header-center" class:expanded={searchFocused}>
		<div class="search-wrapper">
			<Search class="search-icon" />
			<input
				type="text"
				placeholder="Search IP, hostname, MAC..."
				class="search-input"
				value={searchQuery}
				oninput={(e) => onSearchChange?.(e.currentTarget.value)}
				onfocus={() => searchFocused = true}
				onblur={() => searchFocused = false}
			/>
		</div>
	</div>

	<div class="header-right">
		<Button variant="ghost" size="icon" class="settings-btn">
			<Settings class="h-4 w-4" />
		</Button>
		<Button
			class="scan-btn"
			onclick={onScan}
			disabled={isScanning}
		>
			{#if isScanning}
				<Loader class="h-4 w-4 animate-spin" />
				<span>{Math.round(scanProgress)}%</span>
			{:else}
				<span>Scan</span>
			{/if}
		</Button>
	</div>
</header>

<style>
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 56px;
		padding: 0 24px;
		background: rgba(10, 10, 10, 0.8);
		backdrop-filter: blur(12px);
		border-bottom: 1px solid #1a1a1a;
		position: sticky;
		top: 0;
		z-index: 30;
	}

	.header-left {
		display: flex;
		align-items: center;
	}

	.logo {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.logo-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #00d4ff;
		box-shadow: 0 0 8px rgba(0, 212, 255, 0.5);
	}

	.logo-text {
		font-size: 15px;
		font-weight: 600;
		color: #fff;
		letter-spacing: -0.02em;
	}

	.header-center {
		flex: 1;
		max-width: 400px;
		margin: 0 24px;
		transition: max-width 0.2s ease;
	}

	.header-center.expanded {
		max-width: 500px;
	}

	.search-wrapper {
		position: relative;
		width: 100%;
	}

	.search-icon {
		position: absolute;
		left: 12px;
		top: 50%;
		transform: translateY(-50%);
		width: 16px;
		height: 16px;
		color: #666;
		pointer-events: none;
	}

	.search-input {
		width: 100%;
		height: 36px;
		padding: 0 12px 0 40px;
		background: #1a1a1a;
		border: 1px solid #222;
		border-radius: 8px;
		font-size: 14px;
		color: #fff;
		transition: border-color 0.2s, background 0.2s;
	}

	.search-input::placeholder {
		color: #666;
	}

	.search-input:focus {
		outline: none;
		border-color: #00d4ff;
		background: #141414;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.settings-btn {
		color: #666;
	}

	.settings-btn:hover {
		color: #fff;
	}

	.scan-btn {
		min-width: 80px;
		background: #00d4ff;
		color: #000;
		font-weight: 600;
	}

	.scan-btn:hover:not(:disabled) {
		background: #00b8e0;
	}

	.scan-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.scan-btn :global(svg) {
		margin-right: 6px;
	}
</style>
```

---

## Task 13: Create Layout Index

**Files:**
- Create: `apps/web/src/lib/components/layout/index.ts`

**Step 1: Create index**

```typescript
// apps/web/src/lib/components/layout/index.ts
export { default as Header } from './Header.svelte';
```

---

## Task 14: Update Main Page

**Files:**
- Modify: `apps/web/src/routes/+page.svelte`

**Step 1: Replace page content**

```svelte
<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { devicesStore, scanStore, settingsStore } from '$lib/stores';
	import { Header } from '$lib/components/layout';
	import { DevicePanel } from '$lib/components/device';
	import { TopologyCanvas } from '$lib/components/topology';

	// Local state
	let searchQuery = $state('');
	let selectedDeviceId = $state<string | null>(null);

	// Derived
	let devices = $derived(devicesStore.filteredDevices);
	let stats = $derived(devicesStore.stats);
	let isScanning = $derived(scanStore.isScanning);
	let scanProgress = $derived(scanStore.progressPercent);
	let selectedDevice = $derived(selectedDeviceId ? devicesStore.getById(selectedDeviceId) : null);

	onMount(() => {
		devicesStore.load();
	});

	function startScan() {
		scanStore.start(settingsStore.settings.scanDefaults);
	}

	function handleNodeClick(deviceId: string) {
		selectedDeviceId = deviceId;
	}

	function handleNodeDoubleClick(deviceId: string) {
		const device = devicesStore.getById(deviceId);
		if (device) {
			goto(`/device/${encodeURIComponent(device.ip)}`);
		}
	}

	function closePanel() {
		selectedDeviceId = null;
	}

	function handleSearchChange(query: string) {
		searchQuery = query;
		devicesStore.setFilters({ query });
	}
</script>

<div class="app">
	<Header
		isScanning={isScanning}
		scanProgress={scanProgress}
		onScan={startScan}
		{searchQuery}
		onSearchChange={handleSearchChange}
	/>

	<main class="main">
		<!-- Topology Canvas -->
		<div class="canvas-container">
			{#if devices.length > 0}
				<TopologyCanvas
					devices={devices}
					onNodeClick={handleNodeClick}
					onNodeDoubleClick={handleNodeDoubleClick}
				/>
			{:else}
				<div class="empty-state">
					<div class="empty-icon">
						<!-- Network icon placeholder -->
					</div>
					<h2>No devices found</h2>
					<p>Start a scan to discover devices on your network</p>
					<button class="scan-cta" onclick={startScan} disabled={isScanning}>
						{isScanning ? 'Scanning...' : 'Start Scan'}
					</button>
				</div>
			{/if}
		</div>

		<!-- Floating stats -->
		{#if devices.length > 0}
			<div class="stats-bar">
				<span>{stats.total} devices</span>
				<span class="dot">•</span>
				<span class="online">{stats.online} online</span>
				<span class="dot">•</span>
				<span class="offline">{stats.offline} offline</span>
			</div>
		{/if}
	</main>

	<!-- Device detail panel -->
	<DevicePanel
		device={selectedDevice ?? null}
		onClose={closePanel}
	/>
</div>

<style>
	.app {
		min-height: 100vh;
		background: #0a0a0a;
	}

	.main {
		position: relative;
		height: calc(100vh - 56px);
	}

	.canvas-container {
		width: 100%;
		height: 100%;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		text-align: center;
		color: #666;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: #1a1a1a;
		margin-bottom: 24px;
	}

	.empty-state h2 {
		font-size: 24px;
		font-weight: 600;
		color: #fff;
		margin: 0 0 8px;
	}

	.empty-state p {
		font-size: 14px;
		margin: 0 0 24px;
	}

	.scan-cta {
		padding: 12px 32px;
		background: #00d4ff;
		color: #000;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s;
	}

	.scan-cta:hover:not(:disabled) {
		background: #00b8e0;
	}

	.scan-cta:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.stats-bar {
		position: absolute;
		bottom: 24px;
		left: 24px;
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		color: #666;
		background: rgba(20, 20, 20, 0.9);
		backdrop-filter: blur(8px);
		padding: 10px 16px;
		border-radius: 8px;
		border: 1px solid #222;
	}

	.dot {
		opacity: 0.5;
	}

	.online {
		color: #00d4ff;
	}

	.offline {
		color: #404040;
	}
</style>
```

---

## Task 15: Update Color Variables

**Files:**
- Modify: `apps/web/src/app.css`

**Step 1: Update CSS variables**

Add these variables to the existing `:root` section:

```css
:root {
	/* Existing variables... */

	/* PeterParker Topology Colors */
	--pp-bg: 0 0% 4%;           /* #0a0a0a */
	--pp-surface: 0 0% 8%;      /* #141414 */
	--pp-surface-highlight: 0 0% 10%;  /* #1a1a1a */
	--pp-border: 0 0% 13%;      /* #222222 */
	--pp-text-muted: 0 0% 40%;  /* #666666 */
	--pp-accent: 190 100% 50%;  /* #00d4ff */
	--pp-accent-glow: 190 100% 50%; /* For glow effects */
	--pp-success: 142 76% 36%;  /* #22c55e */
	--pp-online: 190 100% 50%;  /* #00d4ff */
	--pp-offline: 0 0% 25%;     /* #404040 */
}
```

---

## Task 16: Verify and Test

**Step 1: Run type check**

Run:
```bash
cd apps/web && pnpm check
```

Expected: No errors

**Step 2: Run dev server**

Run:
```bash
cd apps/web && pnpm dev
```

Expected: Dev server starts, open http://localhost:5173

**Step 3: Visual verification**

1. Open the app in browser
2. Verify empty state shows
3. Click "Start Scan" (mock data should populate)
4. Verify topology canvas renders with nodes
5. Hover a node - tooltip should appear
6. Click a node - panel should slide in
7. Drag a node - it should move
8. Use scroll wheel - canvas should zoom

---

## Task 17: Final Cleanup

**Step 1: Remove unused imports from old page**

If there are unused imports from the old card-based page, remove them.

**Step 2: Verify no console errors**

Open browser dev tools, check for any console errors or warnings.

**Step 3: Test responsive behavior**

Resize browser window, verify canvas resizes properly.

---

## Summary

This implementation plan covers:

1. **D3.js integration** - Force-directed layout for network topology
2. **Topology store** - State management for nodes, edges, positions
3. **Canvas components** - TopologyCanvas, TopologyNode, TopologyEdge, TopologyTooltip
4. **Device panel** - Slide-in detail view
5. **Header** - Minimal header with search and scan button
6. **Main page** - Integration of all components

Each task is designed to be completed in 2-5 minutes with clear verification steps.
