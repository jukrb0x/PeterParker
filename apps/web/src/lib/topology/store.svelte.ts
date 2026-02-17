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
