import type { Device, ScanConfig, ScanProgress, ScanResult } from './types';

/**
 * Mock scanner client for web-only development (without Rust backend)
 */

const MOCK_DEVICES: Device[] = [
	{
		id: '1',
		ip: '192.168.1.1',
		mac: '00:11:22:33:44:55',
		vendor: 'TP-Link',
		hostname: 'Router',
		os: { name: 'Linux', family: 'linux', version: null, confidence: 90, cpe: [] },
		deviceType: 'router',
		firstSeen: new Date().toISOString(),
		lastSeen: new Date().toISOString(),
		isOnline: true,
		ports: [{ number: 80, protocol: 'tcp', state: 'open', service: { name: 'http', version: null, product: null, extraInfo: {} }, banner: null }],
		metadata: { httpTitle: 'TP-LINK Router', httpServer: 'nginx', sshVersion: null, smbInfo: null, ttl: 64, windowSize: null }
	},
	{
		id: '2',
		ip: '192.168.1.101',
		mac: 'AA:BB:CC:DD:EE:FF',
		vendor: 'Apple',
		hostname: 'MacBook-Pro',
		os: { name: 'macOS', family: 'macos', version: '14.0', confidence: 95, cpe: [] },
		deviceType: 'laptop',
		firstSeen: new Date().toISOString(),
		lastSeen: new Date().toISOString(),
		isOnline: true,
		ports: [{ number: 22, protocol: 'tcp', state: 'open', service: { name: 'ssh', version: '9.0', product: 'OpenSSH', extraInfo: {} }, banner: null }],
		metadata: { httpTitle: null, httpServer: null, sshVersion: 'OpenSSH_9.0', smbInfo: null, ttl: 64, windowSize: null }
	},
	{
		id: '3',
		ip: '192.168.1.102',
		mac: '11:22:33:44:55:66',
		vendor: 'Xiaomi',
		hostname: 'Redmi-Phone',
		os: { name: 'Android', family: 'linux', version: '14', confidence: 80, cpe: [] },
		deviceType: 'mobile',
		firstSeen: new Date().toISOString(),
		lastSeen: new Date(Date.now() - 3600000).toISOString(),
		isOnline: false,
		ports: [],
		metadata: { httpTitle: null, httpServer: null, sshVersion: null, smbInfo: null, ttl: null, windowSize: null }
	}
];

class MockScannerClient {
	private devices: Device[] = [...MOCK_DEVICES];
	private scanProgress: Map<string, ScanProgress> = new Map();
	private scanResults: Map<string, ScanResult> = new Map();

	async startScan(config: ScanConfig): Promise<string> {
		const scanId = 'mock-scan-' + Date.now();
		const progress: ScanProgress = {
			scanId,
			status: 'running',
			totalHosts: 254,
			scannedHosts: 0,
			foundDevices: 0,
			currentHost: null,
			eta: 30,
			error: null
		};
		this.scanProgress.set(scanId, progress);

		// Simulate scan progress
		setTimeout(() => this.simulateScan(scanId), 100);
		
		return scanId;
	}

	private async simulateScan(scanId: string) {
		const progress = this.scanProgress.get(scanId)!;
		
		for (let i = 0; i <= 10; i++) {
			await new Promise(r => setTimeout(r, 500));
			progress.scannedHosts = Math.floor((i / 10) * 254);
			progress.foundDevices = Math.floor((i / 10) * this.devices.length);
			progress.eta = Math.floor((10 - i) * 0.5);
		}
		
		progress.status = 'completed';
		progress.scannedHosts = 254;
		progress.foundDevices = this.devices.length;
		progress.eta = null;
		
		this.scanResults.set(scanId, {
			scanId,
			config: { targetRange: '192.168.1.0/24', ports: 'top100', scanType: 'comprehensive', timeout: 2000, concurrency: 50, enableOsDetection: true, enableServiceDetection: true },
			devices: this.devices,
			startedAt: new Date().toISOString(),
			completedAt: new Date().toISOString()
		});
	}

	async getScanProgress(scanId: string): Promise<ScanProgress> {
		return this.scanProgress.get(scanId) ?? {
			scanId,
			status: 'error',
			totalHosts: 0,
			scannedHosts: 0,
			foundDevices: 0,
			currentHost: null,
			eta: null,
			error: 'Scan not found'
		};
	}

	async pauseScan(): Promise<void> {}
	async resumeScan(): Promise<void> {}
	async cancelScan(): Promise<void> {}

	async getScanResult(scanId: string): Promise<ScanResult> {
		return this.scanResults.get(scanId) ?? {
			scanId,
			config: { targetRange: '192.168.1.0/24', ports: 'top100', scanType: 'comprehensive', timeout: 2000, concurrency: 50, enableOsDetection: true, enableServiceDetection: true },
			devices: [],
			startedAt: new Date().toISOString(),
			completedAt: null
		};
	}

	async getDevices(): Promise<Device[]> {
		return this.devices;
	}

	async getDevice(ip: string): Promise<Device | null> {
		return this.devices.find(d => d.ip === ip) ?? null;
	}

	async deleteDevice(): Promise<void> {}

	async rescanDevice(ip: string): Promise<Device> {
		const device = await this.getDevice(ip);
		if (!device) throw new Error('Device not found');
		return device;
	}

	async exportDevices(): Promise<string> {
		return JSON.stringify(this.devices, null, 2);
	}

	async getNetworkInfo(): Promise<{ interface: string; ip: string; netmask: string }> {
		return { interface: 'eth0', ip: '192.168.1.105', netmask: '255.255.255.0' };
	}
}

export const mockScannerClient = new MockScannerClient();
