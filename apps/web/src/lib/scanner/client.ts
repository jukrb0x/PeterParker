import { browser } from '$app/environment';
import type { Device, ScanConfig, ScanProgress, ScanResult } from './types';
import { DeviceType, PortState, ScanStatus, ScanMethod } from './types';

/**
 * Detect if running in Tauri environment
 */
function isTauri(): boolean {
	if (!browser) return false;
	return typeof window !== 'undefined' && '__TAURI__' in window;
}

/**
 * Mock data for browser-only development
 */
const mockDevices: Device[] = [
	{
		id: '1',
		ip: '192.168.1.1',
		mac: 'A0:F3:C1:12:34:56',
		vendor: 'TP-Link',
		hostname: 'router.local',
		os: null,
		deviceType: DeviceType.Router,
		firstSeen: new Date().toISOString(),
		lastSeen: new Date().toISOString(),
		isOnline: true,
		ports: [
			{ number: 80, protocol: 'tcp', state: PortState.Open, service: { name: 'http', version: null, product: null, extraInfo: {} }, banner: null },
			{ number: 443, protocol: 'tcp', state: PortState.Open, service: { name: 'https', version: null, product: null, extraInfo: {} }, banner: null }
		],
		metadata: { httpTitle: 'TP-Link Router', httpServer: 'nginx', sshVersion: null, smbInfo: null, ttl: 64, windowSize: null }
	},
	{
		id: '2',
		ip: '192.168.1.100',
		mac: '00:1B:63:AB:CD:EF',
		vendor: 'Apple',
		hostname: 'MacBook-Pro.local',
		os: { name: 'macOS', family: 'macos', version: '14.0', confidence: 90, cpe: [] },
		deviceType: DeviceType.Laptop,
		firstSeen: new Date().toISOString(),
		lastSeen: new Date().toISOString(),
		isOnline: true,
		ports: [
			{ number: 22, protocol: 'tcp', state: PortState.Open, service: { name: 'ssh', version: '9.0', product: 'OpenSSH', extraInfo: {} }, banner: null }
		],
		metadata: { httpTitle: null, httpServer: null, sshVersion: 'OpenSSH_9.0', smbInfo: null, ttl: 64, windowSize: null }
	},
	{
		id: '3',
		ip: '192.168.1.105',
		mac: '3C:8B:FE:12:34:56',
		vendor: 'Xiaomi',
		hostname: null,
		os: null,
		deviceType: DeviceType.Mobile,
		firstSeen: new Date().toISOString(),
		lastSeen: new Date().toISOString(),
		isOnline: true,
		ports: [],
		metadata: { httpTitle: null, httpServer: null, sshVersion: null, smbInfo: null, ttl: null, windowSize: null }
	},
	{
		id: '4',
		ip: '192.168.1.200',
		mac: null,
		vendor: null,
		hostname: null,
		os: null,
		deviceType: DeviceType.Unknown,
		firstSeen: new Date(Date.now() - 86400000).toISOString(),
		lastSeen: new Date(Date.now() - 3600000).toISOString(),
		isOnline: false,
		ports: [],
		metadata: { httpTitle: null, httpServer: null, sshVersion: null, smbInfo: null, ttl: null, windowSize: null }
	}
];

let mockScanId: string | null = null;
let mockScanProgress: ScanProgress | null = null;

/**
 * Tauri Scanner Client
 * Uses @tauri-apps/api/core invoke() for IPC with Rust backend
 */
class TauriScannerClient {
	private invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

	constructor() {
		// Dynamically import Tauri API
		// We use a lazy loader to avoid issues during SSR
		this.invoke = async (cmd: string, args?: Record<string, unknown>) => {
			if (!isTauri()) {
				throw new Error('Not running in Tauri environment');
			}
			const { invoke } = await import('@tauri-apps/api/core');
			return invoke(cmd, args);
		};
	}

	async startScan(config: ScanConfig): Promise<string> {
		// Map frontend ScanConfig to backend format
		const backendConfig = {
			targetRange: config.targetRange,
			ports: this.mapPortSelection(config.ports),
			scanType: config.scanType,
			timeout: config.timeout,
			concurrency: config.concurrency,
			enableOsDetection: config.enableOsDetection,
			enableServiceDetection: config.enableServiceDetection
		};
		return this.invoke<string>('start_scan', { config: backendConfig });
	}

	private mapPortSelection(ports: ScanConfig['ports']): string {
		if (typeof ports === 'string') {
			return ports;
		}
		// For custom ports array, we need to handle differently
		return 'Top100';
	}

	async getScanProgress(scanId: string): Promise<ScanProgress> {
		const progress = await this.invoke<ScanProgress>('get_scan_progress', { scanId });
		return this.mapProgress(progress);
	}

	private mapProgress(progress: ScanProgress): ScanProgress {
		// Ensure status matches frontend enum values
		return {
			...progress,
			status: progress.status.toLowerCase() as ScanProgress['status']
		};
	}

	async pauseScan(scanId: string): Promise<void> {
		return this.invoke<void>('pause_scan', { scanId });
	}

	async resumeScan(scanId: string): Promise<void> {
		return this.invoke<void>('resume_scan', { scanId });
	}

	async cancelScan(scanId: string): Promise<void> {
		return this.invoke<void>('cancel_scan', { scanId });
	}

	async getScanResult(scanId: string): Promise<ScanResult> {
		const result = await this.invoke<ScanResult>('get_scan_result', { scanId });
		return this.mapResult(result);
	}

	private mapResult(result: ScanResult): ScanResult {
		return {
			...result,
			devices: result.devices.map(this.mapDevice)
		};
	}

	private mapDevice(device: Device): Device {
		return {
			...device,
			deviceType: device.deviceType.toLowerCase() as Device['deviceType']
		};
	}

	async getDevices(): Promise<Device[]> {
		const devices = await this.invoke<Device[]>('get_devices');
		return devices.map(this.mapDevice);
	}

	async getDevice(ip: string): Promise<Device | null> {
		const device = await this.invoke<Device | null>('get_device', { ip });
		return device ? this.mapDevice(device) : null;
	}

	async deleteDevice(id: string): Promise<void> {
		return this.invoke<void>('delete_device', { id });
	}

	async rescanDevice(ip: string): Promise<Device> {
		const device = await this.invoke<Device>('rescan_device', { ip });
		return this.mapDevice(device);
	}

	async exportDevices(): Promise<string> {
		return this.invoke<string>('export_devices');
	}

	async getNetworkInfo(): Promise<{ interface: string; ip: string; netmask: string }> {
		return this.invoke<{ interface: string; ip: string; netmask: string }>('get_network_info');
	}
}

/**
 * Mock Scanner Client
 * Used for browser-only development without Tauri
 */
class MockScannerClient {
	async startScan(config: ScanConfig): Promise<string> {
		mockScanId = `mock-scan-${Date.now()}`;
		mockScanProgress = {
			scanId: mockScanId,
			status: ScanStatus.Running,
			totalHosts: 254,
			scannedHosts: 0,
			foundDevices: 0,
			currentHost: null,
			eta: null,
			error: null
		};
		return mockScanId;
	}

	async getScanProgress(scanId: string): Promise<ScanProgress> {
		if (!mockScanProgress || mockScanProgress.scanId !== scanId) {
			throw new Error('Scan not found');
		}

		// Simulate progress
		if (mockScanProgress.status === ScanStatus.Running) {
			mockScanProgress.scannedHosts = Math.min(
				mockScanProgress.scannedHosts + 10,
				mockScanProgress.totalHosts
			);
			mockScanProgress.foundDevices = Math.floor(mockScanProgress.scannedHosts / 60);

			if (mockScanProgress.scannedHosts >= mockScanProgress.totalHosts) {
				mockScanProgress.status = ScanStatus.Completed;
				mockScanProgress.scannedHosts = mockScanProgress.totalHosts;
			}
		}

		return { ...mockScanProgress };
	}

	async pauseScan(): Promise<void> {
		if (mockScanProgress) {
			mockScanProgress.status = ScanStatus.Paused;
		}
	}

	async resumeScan(): Promise<void> {
		if (mockScanProgress) {
			mockScanProgress.status = ScanStatus.Running;
		}
	}

	async cancelScan(): Promise<void> {
		mockScanId = null;
		mockScanProgress = null;
	}

	async getScanResult(scanId: string): Promise<ScanResult> {
		if (!mockScanId || mockScanId !== scanId) {
			throw new Error('Scan result not found');
		}
		return {
			scanId,
			config: {
				targetRange: '192.168.1.0/24',
				ports: 'top100',
				scanType: ScanMethod.Comprehensive,
				timeout: 2000,
				concurrency: 50,
				enableOsDetection: true,
				enableServiceDetection: true
			},
			devices: mockDevices,
			startedAt: new Date().toISOString(),
			completedAt: new Date().toISOString()
		};
	}

	async getDevices(): Promise<Device[]> {
		return [...mockDevices];
	}

	async getDevice(ip: string): Promise<Device | null> {
		return mockDevices.find((d) => d.ip === ip) || null;
	}

	async deleteDevice(): Promise<void> {
		// Mock: no-op
	}

	async rescanDevice(ip: string): Promise<Device> {
		const device = mockDevices.find((d) => d.ip === ip);
		if (!device) throw new Error('Device not found');
		return device;
	}

	async exportDevices(): Promise<string> {
		return JSON.stringify(mockDevices, null, 2);
	}

	async getNetworkInfo(): Promise<{ interface: string; ip: string; netmask: string }> {
		return { interface: 'en0', ip: '192.168.1.105', netmask: '255.255.255.0' };
	}
}

// Create singleton instance based on environment
function createScannerClient() {
	if (isTauri()) {
		return new TauriScannerClient();
	}
	return new MockScannerClient();
}

// Export singleton instance
// We need to create it lazily because Tauri detection requires browser
let _client: TauriScannerClient | MockScannerClient | null = null;

export const scannerClient = new Proxy({} as TauriScannerClient | MockScannerClient, {
	get(_target, prop) {
		if (!_client) {
			_client = createScannerClient();
		}
		return Reflect.get(_client, prop, _client);
	}
});
