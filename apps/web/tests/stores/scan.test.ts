import { describe, it, expect, vi } from 'vitest';
import { scanStore } from '$lib/stores/scan.svelte';
import { scannerClient } from '$lib/scanner/client';

// Mock scanner client
vi.mock('$lib/scanner/client', () => ({
	scannerClient: {
		startScan: vi.fn().mockResolvedValue('test-scan-id'),
		getScanProgress: vi.fn().mockResolvedValue({
			scanId: 'test-scan-id',
			status: 'completed',
			totalHosts: 254,
			scannedHosts: 254,
			foundDevices: 5,
			currentHost: null,
			eta: null,
			error: null
		}),
		pauseScan: vi.fn().mockResolvedValue(undefined),
		resumeScan: vi.fn().mockResolvedValue(undefined),
		cancelScan: vi.fn().mockResolvedValue(undefined),
		getScanResult: vi.fn().mockResolvedValue({
			scanId: 'test-scan-id',
			config: {},
			devices: [],
			startedAt: new Date().toISOString(),
			completedAt: new Date().toISOString()
		})
	}
}));

describe('scanStore', () => {
	it('starts a scan', async () => {
		const config = {
			targetRange: '192.168.1.0/24',
			ports: 'top100' as const,
			scanType: 'comprehensive' as const,
			timeout: 2000,
			concurrency: 50,
			enableOsDetection: true,
			enableServiceDetection: true
		};

		await scanStore.start(config);
		
		expect(scanStore.scanId).toBe('test-scan-id');
		expect(scannerClient.startScan).toHaveBeenCalledWith(config);
	});

	it('calculates progress percentage', () => {
		scanStore.progress = {
			scanId: 'test',
			status: 'running' as const,
			totalHosts: 100,
			scannedHosts: 50,
			foundDevices: 5,
			currentHost: '192.168.1.1',
			eta: 60,
			error: null
		};

		expect(scanStore.progressPercent).toBe(50);
	});

	it('resets state', () => {
		scanStore.reset();
		
		expect(scanStore.scanId).toBeNull();
		expect(scanStore.progress).toBeNull();
		expect(scanStore.error).toBeNull();
	});
});
