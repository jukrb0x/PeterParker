/**
 * Devices store - Svelte 5 runes
 * Reactive state management for discovered devices
 */

import type { Device, FilterState, DeviceType } from '$lib/scanner/types';
import { scannerClient } from '$lib/scanner/client';

export type { Device, FilterState, DeviceType };

class DevicesStore {
	// State
	devices = $state<Device[]>([]);
	isLoading = $state(false);
	error = $state<string | null>(null);

	// Filters
	filters = $state<FilterState>({
		query: '',
		type: 'all',
		status: 'all',
		sortBy: 'lastSeen',
		sortOrder: 'desc'
	});

	// Derived: filtered and sorted devices
	filteredDevices = $derived(this.applyFilters(this.devices, this.filters));

	// Stats
	stats = $derived({
		total: this.devices.length,
		online: this.devices.filter(d => d.isOnline).length,
		offline: this.devices.filter(d => !d.isOnline).length,
		byType: this.groupByType(this.devices)
	});

	private applyFilters(devices: Device[], filters: FilterState): Device[] {
		let result = [...devices];

		// Text search
		if (filters.query) {
			const q = filters.query.toLowerCase();
			result = result.filter(d =>
				d.ip.toLowerCase().includes(q) ||
				d.mac?.toLowerCase().includes(q) ||
				d.hostname?.toLowerCase().includes(q) ||
				d.vendor?.toLowerCase().includes(q)
			);
		}

		// Type filter
		if (filters.type !== 'all') {
			result = result.filter(d => d.deviceType === filters.type);
		}

		// Status filter
		if (filters.status === 'online') {
			result = result.filter(d => d.isOnline);
		} else if (filters.status === 'offline') {
			result = result.filter(d => !d.isOnline);
		}

		// Sort
		result.sort((a, b) => {
			let comparison = 0;
			switch (filters.sortBy) {
				case 'ip':
					comparison = this.compareIp(a.ip, b.ip);
					break;
				case 'hostname':
					comparison = (a.hostname || '').localeCompare(b.hostname || '');
					break;
				case 'lastSeen':
					comparison = new Date(b.lastSeen).getTime() - new Date(a.lastSeen).getTime();
					break;
				case 'vendor':
					comparison = (a.vendor || '').localeCompare(b.vendor || '');
					break;
			}
			return filters.sortOrder === 'desc' ? -comparison : comparison;
		});

		return result;
	}

	private groupByType(devices: Device[]): Record<DeviceType, number> {
		return devices.reduce((acc, d) => {
			acc[d.deviceType] = (acc[d.deviceType] || 0) + 1;
			return acc;
		}, {} as Record<DeviceType, number>);
	}

	private compareIp(a: string, b: string): number {
		const partsA = a.split('.').map(Number);
		const partsB = b.split('.').map(Number);
		for (let i = 0; i < 4; i++) {
			if (partsA[i] !== partsB[i]) return partsA[i] - partsB[i];
		}
		return 0;
	}

	// Actions
	async load(): Promise<void> {
		this.isLoading = true;
		this.error = null;
		try {
			this.devices = await scannerClient.getDevices();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load devices';
		} finally {
			this.isLoading = false;
		}
	}

	add(device: Device): void {
		const existing = this.devices.findIndex(d => d.id === device.id);
		if (existing >= 0) {
			this.devices[existing] = device;
		} else {
			this.devices.push(device);
		}
	}

	update(id: string, updates: Partial<Device>): void {
		const index = this.devices.findIndex(d => d.id === id);
		if (index >= 0) {
			this.devices[index] = { ...this.devices[index], ...updates };
		}
	}

	remove(id: string): void {
		this.devices = this.devices.filter(d => d.id !== id);
	}

	setFilters(filters: Partial<FilterState>): void {
		this.filters = { ...this.filters, ...filters };
	}

	clearFilters(): void {
		this.filters = {
			query: '',
			type: 'all',
			status: 'all',
			sortBy: 'lastSeen',
			sortOrder: 'desc'
		};
	}

	getByIp(ip: string): Device | undefined {
		return this.devices.find(d => d.ip === ip);
	}

	getById(id: string): Device | undefined {
		return this.devices.find(d => d.id === id);
	}
}

// Singleton instance
export const devicesStore = new DevicesStore();
