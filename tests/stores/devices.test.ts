import { describe, it, expect, vi, beforeEach } from 'vitest';
import { devicesStore } from '$lib/stores/devices.svelte';
import { DeviceType } from '$lib/scanner/types';

// Mock scanner client
vi.mock('$lib/scanner/client', () => ({
	scannerClient: {
		getDevices: vi.fn().mockResolvedValue([
			{
				id: '1',
				ip: '192.168.1.1',
				mac: '00:11:22:33:44:55',
				vendor: 'Apple',
				hostname: null,
				os: null,
				deviceType: DeviceType.Router,
				firstSeen: new Date().toISOString(),
				lastSeen: new Date().toISOString(),
				isOnline: true,
				ports: [],
				metadata: {}
			},
			{
				id: '2',
				ip: '192.168.1.2',
				mac: null,
				vendor: 'Unknown',
				hostname: 'Desktop',
				os: null,
				deviceType: DeviceType.Desktop,
				firstSeen: new Date().toISOString(),
				lastSeen: new Date().toISOString(),
				isOnline: false,
				ports: [],
				metadata: {}
			}
		])
	}
}));

describe('devicesStore', () => {
	beforeEach(() => {
		// Reset store state
		devicesStore.clearFilters();
	});

	it('loads devices from API', async () => {
		await devicesStore.load();
		expect(devicesStore.devices.length).toBe(2);
	});

	it('filters by search query', () => {
		devicesStore.devices = [
			{ id: '1', ip: '192.168.1.1', vendor: 'Apple', deviceType: DeviceType.Router, isOnline: true } as any
		];
		
		devicesStore.setFilters({ query: 'apple' });
		expect(devicesStore.filteredDevices.length).toBe(1);
		
		devicesStore.setFilters({ query: 'samsung' });
		expect(devicesStore.filteredDevices.length).toBe(0);
	});

	it('calculates stats correctly', () => {
		devicesStore.devices = [
			{ id: '1', ip: '192.168.1.1', deviceType: DeviceType.Router, isOnline: true } as any,
			{ id: '2', ip: '192.168.1.2', deviceType: DeviceType.Desktop, isOnline: false } as any,
			{ id: '3', ip: '192.168.1.3', deviceType: DeviceType.Mobile, isOnline: true } as any
		];
		
		expect(devicesStore.stats.total).toBe(3);
		expect(devicesStore.stats.online).toBe(2);
		expect(devicesStore.stats.offline).toBe(1);
	});

	it('adds device to store', () => {
		const device = { 
			id: '4', 
			ip: '192.168.1.4', 
			deviceType: DeviceType.Unknown, 
			isOnline: true 
		} as any;
		
		devicesStore.add(device);
		expect(devicesStore.devices.find(d => d.id === '4')).toBeTruthy();
	});

	it('removes device from store', () => {
		devicesStore.devices = [
			{ id: '1', ip: '192.168.1.1', deviceType: DeviceType.Router, isOnline: true } as any
		];
		
		devicesStore.remove('1');
		expect(devicesStore.devices.length).toBe(0);
	});
});
