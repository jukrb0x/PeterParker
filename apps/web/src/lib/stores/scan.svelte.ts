/**
 * Scan store - Svelte 5 runes
 * Manages active scan state and progress
 */

import type { ScanConfig, ScanProgress, ScanStatus, ScanResult, Device } from '$lib/scanner/types';
import { scannerClient } from '$lib/scanner/client';
import { devicesStore } from './devices.svelte';
import { browser } from '$app/environment';

export type { ScanConfig, ScanProgress };

class ScanStore {
	// Active scan state
	scanId = $state<string | null>(null);
	config = $state<ScanConfig | null>(null);
	progress = $state<ScanProgress | null>(null);
	result = $state<ScanResult | null>(null);

	// UI state
	isStarting = $state(false);
	error = $state<string | null>(null);

	// Derived states
	isScanning = $derived(this.progress?.status === 'running');
	isPaused = $derived(this.progress?.status === 'paused');
	progressPercent = $derived(
		this.progress && this.progress.totalHosts > 0
			? Math.round((this.progress.scannedHosts / this.progress.totalHosts) * 100)
			: 0
	);

	private pollInterval: ReturnType<typeof setInterval> | null = null;
	private unlistenFns: (() => void)[] = [];

	async start(config: ScanConfig): Promise<void> {
		this.isStarting = true;
		this.error = null;
		this.result = null;

		try {
			this.scanId = await scannerClient.startScan(config);
			this.config = config;
			
			// Set up event listeners for Tauri
			await this.setupEventListeners();
			
			// Also start polling as fallback
			this.startPolling();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to start scan';
		} finally {
			this.isStarting = false;
		}
	}

	private async setupEventListeners(): Promise<void> {
		if (!browser) return;
		
		// Check if we're in Tauri environment
		if (!('__TAURI__' in window)) return;
		
		try {
			const { listen } = await import('@tauri-apps/api/event');
			
			// Listen for scan progress events
			const unlistenProgress = await listen<{
				scanId: string;
				scanned: number;
				total: number;
				found: number;
				current: string | null;
			}>('scan-progress', (event) => {
				if (event.payload.scanId !== this.scanId) return;
				
				this.progress = {
					scanId: event.payload.scanId,
					status: 'running',
					totalHosts: event.payload.total,
					scannedHosts: event.payload.scanned,
					foundDevices: event.payload.found,
					currentHost: event.payload.current,
					eta: null,
					error: null
				};
			});
			this.unlistenFns.push(unlistenProgress);
			
			// Listen for device found events
			const unlistenDevice = await listen<{
				scanId: string;
				device: Device;
			}>('device-found', (event) => {
				if (event.payload.scanId !== this.scanId) return;
				devicesStore.add(event.payload.device);
			});
			this.unlistenFns.push(unlistenDevice);
			
			// Listen for scan completion
			const unlistenCompleted = await listen<{
				scanId: string;
				devices: number;
			}>('scan-completed', async (event) => {
				if (event.payload.scanId !== this.scanId) return;
				
				this.stopPolling();
				await this.loadResult();
			});
			this.unlistenFns.push(unlistenCompleted);
			
			// Listen for errors
			const unlistenError = await listen<{
				scanId: string;
				error: string;
			}>('scan-error', (event) => {
				if (event.payload.scanId !== this.scanId) return;
				
				this.error = event.payload.error;
				this.stopPolling();
			});
			this.unlistenFns.push(unlistenError);
			
		} catch (e) {
			console.warn('Failed to set up Tauri event listeners:', e);
		}
	}

	private cleanupEventListeners(): void {
		for (const unlisten of this.unlistenFns) {
			unlisten();
		}
		this.unlistenFns = [];
	}

	async pause(): Promise<void> {
		if (!this.scanId || !this.isScanning) return;
		try {
			await scannerClient.pauseScan(this.scanId);
			if (this.progress) {
				this.progress = { ...this.progress, status: 'paused' };
			}
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to pause scan';
		}
	}

	async resume(): Promise<void> {
		if (!this.scanId || !this.isPaused) return;
		try {
			await scannerClient.resumeScan(this.scanId);
			if (this.progress) {
				this.progress = { ...this.progress, status: 'running' };
			}
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to resume scan';
		}
	}

	async cancel(): Promise<void> {
		if (!this.scanId) return;
		try {
			await scannerClient.cancelScan(this.scanId);
			this.cleanup();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to cancel scan';
		}
	}

	async rescanDevice(ip: string): Promise<void> {
		try {
			const device = await scannerClient.rescanDevice(ip);
			devicesStore.add(device);
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to rescan device';
		}
	}

	private startPolling(): void {
		this.stopPolling();
		this.pollInterval = setInterval(() => this.updateProgress(), 500);
	}

	private stopPolling(): void {
		if (this.pollInterval) {
			clearInterval(this.pollInterval);
			this.pollInterval = null;
		}
	}

	private async updateProgress(): Promise<void> {
		if (!this.scanId) return;

		try {
			this.progress = await scannerClient.getScanProgress(this.scanId);

			// Auto-stop polling on completion
			if (
				this.progress.status === 'completed' ||
				this.progress.status === 'error'
			) {
				this.stopPolling();
				this.cleanupEventListeners();
				if (this.progress.status === 'completed') {
					await this.loadResult();
				}
			}
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to get progress';
			this.stopPolling();
		}
	}

	private async loadResult(): Promise<void> {
		if (!this.scanId) return;
		try {
			this.result = await scannerClient.getScanResult(this.scanId);
			// Refresh devices list
			await devicesStore.load();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load result';
		}
	}

	private cleanup(): void {
		this.stopPolling();
		this.cleanupEventListeners();
		this.scanId = null;
		this.config = null;
		this.progress = null;
		this.result = null;
	}

	reset(): void {
		this.cleanup();
		this.error = null;
	}
}

// Singleton instance
export const scanStore = new ScanStore();
