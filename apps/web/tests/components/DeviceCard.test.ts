import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import DeviceCard from '$lib/components/device/DeviceCard.svelte';
import { DeviceType } from '$lib/scanner/types';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}));

describe('DeviceCard', () => {
	const mockDevice = {
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
	};

	it('renders device IP', () => {
		render(DeviceCard, { props: { device: mockDevice } });
		expect(screen.getByText('192.168.1.1')).toBeTruthy();
	});

	it('shows vendor name', () => {
		render(DeviceCard, { props: { device: mockDevice } });
		expect(screen.getByText('Apple')).toBeTruthy();
	});

	it('shows online status indicator', () => {
		const { container } = render(DeviceCard, { props: { device: mockDevice } });
		const statusDot = container.querySelector('.bg-pp-online');
		expect(statusDot).toBeTruthy();
	});

	it('shows MAC address', () => {
		render(DeviceCard, { props: { device: mockDevice } });
		expect(screen.getByText('00:11:22:33:44:55')).toBeTruthy();
	});

	it('calls onClick when clicked', async () => {
		const onClick = vi.fn();
		const { container } = render(DeviceCard, { 
			props: { device: mockDevice, onClick } 
		});
		
		const button = container.querySelector('button');
		button?.click();
		
		expect(onClick).toHaveBeenCalled();
	});
});
