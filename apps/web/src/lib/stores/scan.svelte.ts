/**
 * Scan store - Svelte 5 runes
 * Manages active scan state and progress
 */

import type { ScanConfig, ScanProgress, ScanStatus, ScanResult } from '$lib/scanner/types';
import { scannerClient } from '$lib/scanner/client';
import { devicesStore } from './devices.svelte';

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

	async start(config: ScanConfig): Promise<void> {
		this.isStarting = true;
		this.error = null;
		this.result = null;

		try {
			this.scanId = await scannerClient.startScan(config);
			this.config = config;
			this.startPolling();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to start scan';
		} finally {
			this.isStarting = false;
		}
	}

	async pause(): Promise<void> {
		if (!this.scanId || !this.isScanning) return;
		try {
			await scannerClient.pauseScan(this.scanId);
			await this.updateProgress();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to pause scan';
		}
	}

	async resume(): Promise<void> {
		if (!this.scanId || !this.isPaused) return;
		try {
			await scannerClient.resumeScan(this.scanId);
			await this.updateProgress();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to resume scan';
		}
	}

	async cancel(): Promise<void> {
		if (!this.scanId) return;
		try {
			await scannerClient.cancelScan(this.scanId);
			this.stopPolling();
			this.scanId = null;
			this.progress = null;
			this.config = null;
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

	reset(): void {
		this.stopPolling();
		this.scanId = null;
		this.config = null;
		this.progress = null;
		this.result = null;
		this.error = null;
	}
}

// Singleton instance
export const scanStore = new ScanStore();
