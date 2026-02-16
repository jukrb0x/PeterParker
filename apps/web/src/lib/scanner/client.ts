import { invoke } from '@tauri-apps/api/core';
import type { Device, ScanConfig, ScanProgress, ScanResult } from './types';

/**
 * Tauri command wrappers for scanner operations
 */

export const scannerClient = {
	/**
	 * Start a new scan with the given configuration
	 */
	async startScan(config: ScanConfig): Promise<string> {
		return invoke<string>('start_scan', { config });
	},

	/**
	 * Get current scan progress
	 */
	async getScanProgress(scanId: string): Promise<ScanProgress> {
		return invoke<ScanProgress>('get_scan_progress', { scanId });
	},

	/**
	 * Pause an active scan
	 */
	async pauseScan(scanId: string): Promise<void> {
		return invoke('pause_scan', { scanId });
	},

	/**
	 * Resume a paused scan
	 */
	async resumeScan(scanId: string): Promise<void> {
		return invoke('resume_scan', { scanId });
	},

	/**
	 * Cancel an active scan
	 */
	async cancelScan(scanId: string): Promise<void> {
		return invoke('cancel_scan', { scanId });
	},

	/**
	 * Get scan results
	 */
	async getScanResult(scanId: string): Promise<ScanResult> {
		return invoke<ScanResult>('get_scan_result', { scanId });
	},

	/**
	 * Get all discovered devices
	 */
	async getDevices(): Promise<Device[]> {
		return invoke<Device[]>('get_devices');
	},

	/**
	 * Get a single device by IP
	 */
	async getDevice(ip: string): Promise<Device | null> {
		return invoke<Device | null>('get_device', { ip });
	},

	/**
	 * Delete a device from the database
	 */
	async deleteDevice(id: string): Promise<void> {
		return invoke('delete_device', { id });
	},

	/**
	 * Rescan a specific device
	 */
	async rescanDevice(ip: string): Promise<Device> {
		return invoke<Device>('rescan_device', { ip });
	},

	/**
	 * Export devices to JSON
	 */
	async exportDevices(): Promise<string> {
		return invoke<string>('export_devices');
	},

	/**
	 * Get local network interface info
	 */
	async getNetworkInfo(): Promise<{ interface: string; ip: string; netmask: string }> {
		return invoke('get_network_info');
	}
};
