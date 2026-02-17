/**
 * Settings store - Svelte 5 runes
 * Persistent user preferences
 */

import type { ScanConfig } from '$lib/scanner/types';
import { ScanMethod } from '$lib/scanner/types';

interface AppSettings {
	scanDefaults: ScanConfig;
	ui: {
		viewMode: 'grid' | 'list' | 'topology';
		showOffline: boolean;
		refreshInterval: number; // seconds, 0 = off
	};
}

const DEFAULT_SETTINGS: AppSettings = {
	scanDefaults: {
		targetRange: '192.168.1.0/24',
		ports: 'top100',
		scanType: ScanMethod.Comprehensive,
		timeout: 2000,
		concurrency: 50,
		enableOsDetection: true,
		enableServiceDetection: true
	},
	ui: {
		viewMode: 'list',
		showOffline: true,
		refreshInterval: 0
	}
};

class SettingsStore {
	settings = $state<AppSettings>(this.load());

	private load(): AppSettings {
		if (typeof localStorage === 'undefined') return DEFAULT_SETTINGS;
		try {
			const stored = localStorage.getItem('peterparker_settings');
			return stored ? { ...DEFAULT_SETTINGS, ...JSON.parse(stored) } : DEFAULT_SETTINGS;
		} catch {
			return DEFAULT_SETTINGS;
		}
	}

	private save(): void {
		if (typeof localStorage === 'undefined') return;
		try {
			localStorage.setItem('peterparker_settings', JSON.stringify(this.settings));
		} catch {
			// ignore
		}
	}

	updateScanDefaults(updates: Partial<ScanConfig>): void {
		this.settings.scanDefaults = { ...this.settings.scanDefaults, ...updates };
		this.save();
	}

	updateUI(updates: Partial<AppSettings['ui']>): void {
		this.settings.ui = { ...this.settings.ui, ...updates };
		this.save();
	}

	reset(): void {
		this.settings = DEFAULT_SETTINGS;
		this.save();
	}
}

export const settingsStore = new SettingsStore();
