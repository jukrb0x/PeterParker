/**
 * Core type definitions for PeterParker
 * Shared between frontend and backend via Tauri IPC
 */

export enum DeviceType {
	Router = 'router',
	Switch = 'switch',
	Desktop = 'desktop',
	Laptop = 'laptop',
	Mobile = 'mobile',
	Tablet = 'tablet',
	Iot = 'iot',
	Printer = 'printer',
	Nas = 'nas',
	Camera = 'camera',
	Tv = 'tv',
	GameConsole = 'game_console',
	Unknown = 'unknown'
}

export enum PortState {
	Open = 'open',
	Closed = 'closed',
	Filtered = 'filtered'
}

export enum ScanStatus {
	Pending = 'pending',
	Running = 'running',
	Paused = 'paused',
	Completed = 'completed',
	Error = 'error'
}

export enum ScanMethod {
	Arp = 'arp',
	Icmp = 'icmp',
	TcpSyn = 'tcp_syn',
	TcpConnect = 'tcp_connect',
	Comprehensive = 'comprehensive'
}

export interface Port {
	number: number;
	protocol: 'tcp' | 'udp';
	state: PortState;
	service: Service | null;
	banner: string | null;
}

export interface Service {
	name: string;
	version: string | null;
	product: string | null;
	extraInfo: Record<string, unknown>;
}

export interface OperatingSystem {
	name: string;
	family: 'windows' | 'linux' | 'macos' | 'bsd' | 'embedded' | 'unknown';
	version: string | null;
	confidence: number;
	cpe: string[];
}

export interface DeviceMetadata {
	httpTitle: string | null;
	httpServer: string | null;
	sshVersion: string | null;
	smbInfo: Record<string, unknown> | null;
	ttl: number | null;
	windowSize: number | null;
}

export interface Device {
	id: string;
	ip: string;
	mac: string | null;
	vendor: string | null;
	hostname: string | null;
	os: OperatingSystem | null;
	deviceType: DeviceType;
	firstSeen: string; // ISO date
	lastSeen: string; // ISO date
	isOnline: boolean;
	ports: Port[];
	metadata: DeviceMetadata;
}

export interface ScanConfig {
	targetRange: string; // CIDR notation, e.g., "192.168.1.0/24"
	ports: 'top100' | 'top1000' | number[] | 'all';
	scanType: ScanMethod;
	timeout: number; // ms per host
	concurrency: number; // parallel hosts
	enableOsDetection: boolean;
	enableServiceDetection: boolean;
}

export interface ScanProgress {
	scanId: string;
	status: ScanStatus;
	totalHosts: number;
	scannedHosts: number;
	foundDevices: number;
	currentHost: string | null;
	eta: number | null;
	error: string | null;
}

export interface ScanResult {
	scanId: string;
	config: ScanConfig;
	devices: Device[];
	startedAt: string;
	completedAt: string | null;
}

// UI-specific types
export type ViewMode = 'grid' | 'list' | 'topology';

export interface FilterState {
	query: string;
	type: DeviceType | 'all';
	status: 'all' | 'online' | 'offline';
	sortBy: 'ip' | 'hostname' | 'lastSeen' | 'vendor';
	sortOrder: 'asc' | 'desc';
}
