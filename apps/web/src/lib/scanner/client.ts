import { invoke } from '@tauri-apps/api/core';
import type { Device, ScanConfig, ScanProgress, ScanResult } from './types';
import { mockScannerClient } from './client.mock';

/**
 * Detect if running in Tauri environment
 */
const isTauri = typeof window !== 'undefined' && 
	// @ts-ignore
	!!window.__TAURI__;

/**
 * Tauri command wrappers for scanner operations
 */
class TauriScannerClient {
	async startScan(config: ScanConfig): Promise<string> {
		return invoke<string>('start_scan', { config });
	}

	async getScanProgress(scanId: string): Promise<ScanProgress> {
		return invoke<ScanProgress>('get_scan_progress', { scanId });
	}

	async pauseScan(scanId: string): Promise<void> {
		return invoke('pause_scan', { scanId });
	}

	async resumeScan(scanId: string): Promise<void> {
		return invoke('resume_scan', { scanId });
	}

	async cancelScan(scanId: string): Promise<void> {
		return invoke('cancel_scan', { scanId });
	}

	async getScanResult(scanId: string): Promise<ScanResult> {
		return invoke<ScanResult>('get_scan_result', { scanId });
	}

	async getDevices(): Promise<Device[]> {
		return invoke<Device[]>('get_devices');
	}

	async getDevice(ip: string): Promise<Device | null> {
		return invoke<Device | null>('get_device', { ip });
	}

	async deleteDevice(id: string): Promise<void> {
		return invoke('delete_device', { id });
	}

	async rescanDevice(ip: string): Promise<Device> {
		return invoke<Device>('rescan_device', { ip });
	}

	async exportDevices(): Promise<string> {
		return invoke<string>('export_devices');
	}

	async getNetworkInfo(): Promise<{ interface: string; ip: string; netmask: string }> {
		return invoke('get_network_info');
	}
}

/**
 * Use real Tauri client if in Tauri, otherwise use mock
 */
export const scannerClient = isTauri 
	? new TauriScannerClient() 
	: mockScannerClient;

// Also export individual functions for convenience
export const {
	startScan,
	getScanProgress,
	pauseScan,
	resumeScan,
	cancelScan,
	getScanResult,
	getDevices,
	getDevice,
	deleteDevice,
	rescanDevice,
	exportDevices,
	getNetworkInfo
} = scannerClient;
