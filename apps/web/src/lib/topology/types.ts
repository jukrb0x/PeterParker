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
