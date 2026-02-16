import { browser } from '$app/environment';
import type { Device, ScanConfig, ScanProgress, ScanResult } from './types';

const API_BASE = 'http://localhost:3030/api';

/**
 * HTTP Client for Web frontend to connect to Scanner Service
 * Works in any browser environment
 */
class HttpScannerClient {
	async startScan(config: ScanConfig): Promise<string> {
		const res = await fetch(`${API_BASE}/scan`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(config),
		});
		const data = await res.json();
		if (!data.success) throw new Error(data.error);
		return data.data;
	}

	async getScanProgress(scanId: string): Promise<ScanProgress> {
		const res = await fetch(`${API_BASE}/scan/${scanId}`);
		const data = await res.json();
		if (!data.success) throw new Error(data.error);
		return data.data;
	}

	async pauseScan(): Promise<void> {}
	async resumeScan(): Promise<void> {}
	async cancelScan(): Promise<void> {}

	async getScanResult(scanId: string): Promise<ScanResult> {
		return this.getScanProgress(scanId);
	}

	async getDevices(): Promise<Device[]> {
		const res = await fetch(`${API_BASE}/devices`);
		const data = await res.json();
		if (!data.success) throw new Error(data.error);
		return data.data;
	}

	async getDevice(ip: string): Promise<Device | null> {
		const res = await fetch(`${API_BASE}/devices/${ip}`);
		const data = await res.json();
		if (!data.success) return null;
		return data.data;
	}

	async deleteDevice(): Promise<void> {}

	async rescanDevice(ip: string): Promise<Device> {
		const device = await this.getDevice(ip);
		if (!device) throw new Error('Device not found');
		return device;
	}

	async exportDevices(): Promise<string> {
		const devices = await this.getDevices();
		return JSON.stringify(devices, null, 2);
	}

	async getNetworkInfo(): Promise<{ interface: string; ip: string; netmask: string }> {
		return { interface: 'eth0', ip: '192.168.1.105', netmask: '255.255.255.0' };
	}
}

// Export singleton instance
export const scannerClient = new HttpScannerClient();
