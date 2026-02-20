<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { devicesStore, scanStore, settingsStore } from '$lib/stores';
	import { Button } from '$lib/components/ui/button';
	import { Wifi, Search, Grid3X3, List, Network, Settings, Play, Pause, Square, RefreshCw, Download, ChevronDown } from 'lucide-svelte';
	import { TopologyCanvas } from '$lib/components/topology';
	import { DeviceIcon } from '$lib/components/icons';

	// View state
	let viewMode = $state<'grid' | 'list' | 'topology'>('grid');
	let searchQuery = $state('');
	let typeFilter = $state('all');
	let statusFilter = $state('all');
	let showFilters = $state(false);

	// Derived
	let devices = $derived(devicesStore.filteredDevices);
	let allDevices = $derived(devicesStore.devices);
	let stats = $derived(devicesStore.stats);
	let isScanning = $derived(scanStore.isScanning);
	let progress = $derived(scanStore.progress);
	let isLoading = $derived(devicesStore.isLoading);

	// Filter devices based on search and filters
	$effect(() => {
		let filtered = [...allDevices];

		if (searchQuery) {
			const q = searchQuery.toLowerCase();
			filtered = filtered.filter(d =>
				d.ip.toLowerCase().includes(q) ||
				d.mac?.toLowerCase().includes(q) ||
				d.hostname?.toLowerCase().includes(q) ||
				d.vendor?.toLowerCase().includes(q)
			);
		}

		if (typeFilter !== 'all') {
			filtered = filtered.filter(d => d.deviceType === typeFilter);
		}

		if (statusFilter === 'online') {
			filtered = filtered.filter(d => d.isOnline);
		} else if (statusFilter === 'offline') {
			filtered = filtered.filter(d => !d.isOnline);
		}

		// Sort by IP
		filtered.sort((a, b) => {
			const partsA = a.ip.split('.').map(Number);
			const partsB = b.ip.split('.').map(Number);
			for (let i = 0; i < 4; i++) {
				if (partsA[i] !== partsB[i]) return partsA[i] - partsB[i];
			}
			return 0;
		});

		devicesStore.filteredDevices = filtered;
	});

	onMount(() => {
		devicesStore.load();
	});

	function startScan() {
		scanStore.start(settingsStore.settings.scanDefaults);
	}

	function toggleView() {
		if (viewMode === 'grid') viewMode = 'list';
		else if (viewMode === 'list') viewMode = 'topology';
		else viewMode = 'grid';
	}

	function handleNodeClick(deviceId: string) {
		const device = devicesStore.getById(deviceId);
		if (device) goto(`/device/${encodeURIComponent(device.ip)}`);
	}

	const deviceTypes = [
		{ value: 'all', label: 'All Types' },
		{ value: 'router', label: 'Router' },
		{ value: 'desktop', label: 'Desktop' },
		{ value: 'laptop', label: 'Laptop' },
		{ value: 'mobile', label: 'Mobile' },
		{ value: 'iot', label: 'IoT' },
		{ value: 'printer', label: 'Printer' },
		{ value: 'nas', label: 'NAS' },
		{ value: 'unknown', label: 'Unknown' }
	];
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Sora:wght@300;400;500;600;700&display=swap" rel="stylesheet">
</svelte:head>

<div class="app">
	<!-- Header -->
	<header class="header">
		<div class="header-left">
			<div class="logo">
				<span class="logo-icon">◈</span>
				<span class="logo-text">PETERPARKER</span>
			</div>
			<div class="status-badge" class:scanning={isScanning}>
				<span class="status-dot"></span>
				<span>{isScanning ? 'SCANNING' : stats.online + ' ONLINE'}</span>
			</div>
		</div>
		<div class="header-right">
			<div class="view-toggle">
				<button class="toggle-btn" class:active={viewMode === 'grid'} onclick={() => viewMode = 'grid'}>
					<Grid3X3 size={16} />
				</button>
				<button class="toggle-btn" class:active={viewMode === 'list'} onclick={() => viewMode = 'list'}>
					<List size={16} />
				</button>
				<button class="toggle-btn" class:active={viewMode === 'topology'} onclick={() => viewMode = 'topology'}>
					<Network size={16} />
				</button>
			</div>
			<button class="btn-icon" aria-label="Settings">
				<Settings size={18} />
			</button>
			<button class="btn-scan" onclick={startScan} disabled={isScanning}>
				{#if isScanning}
					<RefreshCw size={16} class="spin" />
					<span>SCANNING...</span>
				{:else}
					<Play size={16} />
					<span>SCAN NETWORK</span>
				{/if}
			</button>
		</div>
	</header>

	<!-- Scan Progress Bar -->
	{#if progress && isScanning}
		<div class="progress-bar">
			<div class="progress-fill" style="width: {scanStore.progressPercent}%"></div>
			<div class="progress-info">
				<span class="progress-text">
					{progress.scannedHosts} / {progress.totalHosts} hosts
					{#if progress.currentHost} · {progress.currentHost}{/if}
				</span>
				<span class="progress-found">{progress.foundDevices} found</span>
			</div>
		</div>
	{/if}

	<main class="main">
		<!-- Stats Row -->
		<div class="stats-row">
			<div class="stat">
				<span class="stat-value">{stats.total}</span>
				<span class="stat-label">DEVICES</span>
			</div>
			<div class="stat stat-online">
				<span class="stat-value">{stats.online}</span>
				<span class="stat-label">ONLINE</span>
			</div>
			<div class="stat stat-offline">
				<span class="stat-value">{stats.offline}</span>
				<span class="stat-label">OFFLINE</span>
			</div>
		</div>

		<!-- Toolbar -->
		<div class="toolbar">
			<div class="search-box">
				<Search size={16} class="search-icon" />
				<input
					type="text"
					placeholder="Search by IP, MAC, hostname..."
					bind:value={searchQuery}
				/>
			</div>
			<button class="filter-toggle" onclick={() => showFilters = !showFilters}>
				Filters
				<span class="chevron" class:rotated={showFilters}>
					<ChevronDown size={14} />
				</span>
			</button>
		</div>

		{#if showFilters}
			<div class="filters-panel">
				<div class="filter-group">
					<label for="filter-type">Device Type</label>
					<select id="filter-type" bind:value={typeFilter}>
						{#each deviceTypes as t}
							<option value={t.value}>{t.label}</option>
						{/each}
					</select>
				</div>
				<div class="filter-group">
					<label for="filter-status">Status</label>
					<select id="filter-status" bind:value={statusFilter}>
						<option value="all">All</option>
						<option value="online">Online</option>
						<option value="offline">Offline</option>
					</select>
				</div>
			</div>
		{/if}

		<!-- Content -->
		{#if viewMode === 'topology'}
			<div class="topology-container">
				{#if devices.length > 0}
					<TopologyCanvas
						devices={devices}
						onNodeClick={handleNodeClick}
						onNodeDoubleClick={handleNodeClick}
					/>
				{:else}
					<div class="empty-state">
						<div class="empty-icon">◇</div>
						<h3>No Devices</h3>
						<p>Run a network scan to visualize your topology</p>
						<button class="btn-scan" onclick={startScan} disabled={isScanning}>
							<Play size={16} />
							<span>START SCAN</span>
						</button>
					</div>
				{/if}
			</div>
		{:else if viewMode === 'list'}
			<div class="list-view">
				<div class="list-header">
					<span class="col-status">STATUS</span>
					<span class="col-ip">IP ADDRESS</span>
					<span class="col-hostname">HOSTNAME</span>
					<span class="col-mac">MAC ADDRESS</span>
					<span class="col-vendor">VENDOR</span>
					<span class="col-type">TYPE</span>
				</div>
				<div class="list-body">
					{#each devices as device (device.id)}
						<button class="list-row" onclick={() => goto(`/device/${encodeURIComponent(device.ip)}`)}>
							<span class="col-status">
								<span class="status-indicator" class:online={device.isOnline}></span>
							</span>
							<span class="col-ip">{device.ip}</span>
							<span class="col-hostname">{device.hostname || '—'}</span>
							<span class="col-mac">{device.mac || '—'}</span>
							<span class="col-vendor">{device.vendor || '—'}</span>
							<span class="col-type">{device.deviceType}</span>
						</button>
					{:else}
						{#if isLoading}
							<div class="loading-state">
								<RefreshCw size={24} class="spin" />
								<span>Loading devices...</span>
							</div>
						{:else}
							<div class="empty-state">
								<div class="empty-icon">◇</div>
								<h3>No Devices Found</h3>
								<p>{allDevices.length === 0 ? 'Run a network scan to discover devices' : 'Try adjusting your filters'}</p>
								{#if allDevices.length === 0}
									<button class="btn-scan" onclick={startScan} disabled={isScanning}>
										<Play size={16} />
										<span>START SCAN</span>
									</button>
								{/if}
							</div>
						{/if}
					{/each}
				</div>
			</div>
		{:else}
			<!-- Grid View -->
			<div class="grid-view">
				{#each devices as device (device.id)}
					<button class="device-card" onclick={() => goto(`/device/${encodeURIComponent(device.ip)}`)}>
						<div class="card-header">
							<div class="device-icon" class:online={device.isOnline}>
								<DeviceIcon type={device.deviceType} size={24} color={device.isOnline ? '#00d4ff' : '#666'} />
							</div>
							<span class="device-status" class:online={device.isOnline}></span>
						</div>
						<div class="card-body">
							<span class="device-ip">{device.ip}</span>
							<span class="device-name">{device.hostname || device.vendor || 'Unknown Device'}</span>
							<span class="device-meta">
								<span>{device.deviceType}</span>
								{#if device.ports.length > 0}
									<span>· {device.ports.length} ports</span>
								{/if}
							</span>
						</div>
						{#if device.mac}
							<div class="card-footer">
								<span class="device-mac">{device.mac}</span>
							</div>
						{/if}
					</button>
				{:else}
					{#if isLoading}
						<div class="loading-state full">
							<RefreshCw size={32} class="spin" />
							<span>Loading devices...</span>
						</div>
					{:else}
						<div class="empty-state full">
							<div class="empty-icon">◇</div>
							<h3>No Devices Found</h3>
							<p>{allDevices.length === 0 ? 'Run a network scan to discover devices on your network' : 'Try adjusting your search or filters'}</p>
							{#if allDevices.length === 0}
								<button class="btn-scan" onclick={startScan} disabled={isScanning}>
									<Play size={16} />
									<span>START SCAN</span>
								</button>
							{/if}
						</div>
					{/if}
				{/each}
			</div>
		{/if}
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		background: #08080a;
		color: #e8e8ec;
		font-family: 'Sora', sans-serif;
	}

	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	/* Header */
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 24px;
		height: 64px;
		background: #0c0c0f;
		border-bottom: 1px solid #1a1a1f;
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 24px;
	}

	.logo {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.logo-icon {
		font-size: 20px;
		color: #00d4ff;
		text-shadow: 0 0 20px rgba(0, 212, 255, 0.5);
	}

	.logo-text {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 0.15em;
		color: #fff;
	}

	.status-badge {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 6px 12px;
		background: #0a0a0d;
		border: 1px solid #1a1a1f;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 500;
		letter-spacing: 0.05em;
		color: #888;
	}

	.status-badge.scanning {
		color: #00d4ff;
		border-color: rgba(0, 212, 255, 0.3);
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: #22c55e;
	}

	.status-badge.scanning .status-dot {
		background: #00d4ff;
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; box-shadow: 0 0 0 0 rgba(0, 212, 255, 0.5); }
		50% { opacity: 0.6; box-shadow: 0 0 0 4px rgba(0, 212, 255, 0); }
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.view-toggle {
		display: flex;
		background: #0a0a0d;
		border: 1px solid #1a1a1f;
		border-radius: 6px;
		padding: 2px;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 28px;
		border: none;
		background: transparent;
		color: #666;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.15s;
	}

	.toggle-btn:hover {
		color: #aaa;
	}

	.toggle-btn.active {
		background: #1a1a1f;
		color: #fff;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border: 1px solid #1a1a1f;
		background: transparent;
		color: #888;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.15s;
	}

	.btn-icon:hover {
		background: #1a1a1f;
		color: #fff;
	}

	.btn-scan {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0 20px;
		height: 40px;
		border: none;
		background: #00d4ff;
		color: #000;
		font-family: 'Sora', sans-serif;
		font-size: 12px;
		font-weight: 600;
		letter-spacing: 0.05em;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.15s;
	}

	.btn-scan:hover:not(:disabled) {
		background: #00b8e0;
	}

	.btn-scan:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Progress Bar */
	.progress-bar {
		height: 4px;
		background: #0a0a0d;
		position: relative;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #00d4ff, #00ff88);
		transition: width 0.3s;
	}

	.progress-info {
		display: flex;
		justify-content: space-between;
		padding: 8px 24px;
		background: #0a0a0d;
		font-size: 11px;
		font-family: 'JetBrains Mono', monospace;
		color: #666;
	}

	.progress-found {
		color: #00d4ff;
	}

	/* Main */
	.main {
		flex: 1;
		padding: 24px;
	}

	/* Stats */
	.stats-row {
		display: flex;
		gap: 24px;
		margin-bottom: 24px;
	}

	.stat {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.stat-value {
		font-size: 28px;
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
		color: #fff;
	}

	.stat-label {
		font-size: 10px;
		font-weight: 500;
		letter-spacing: 0.1em;
		color: #666;
	}

	.stat-online .stat-value { color: #00d4ff; }
	.stat-offline .stat-value { color: #555; }

	/* Toolbar */
	.toolbar {
		display: flex;
		gap: 12px;
		margin-bottom: 16px;
	}

	.search-box {
		flex: 1;
		max-width: 400px;
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 0 14px;
		height: 40px;
		background: #0a0a0d;
		border: 1px solid #1a1a1f;
		border-radius: 8px;
	}

	.search-box:focus-within {
		border-color: #333;
	}

	:global(.search-icon) {
		color: #555;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		color: #fff;
		font-family: 'Sora', sans-serif;
		font-size: 13px;
		outline: none;
	}

	.search-box input::placeholder {
		color: #444;
	}

	.filter-toggle {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0 14px;
		height: 40px;
		background: #0a0a0d;
		border: 1px solid #1a1a1f;
		color: #888;
		font-family: 'Sora', sans-serif;
		font-size: 12px;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.15s;
	}

	.filter-toggle:hover {
		background: #1a1a1f;
		color: #fff;
	}

	.filter-toggle .chevron {
		display: flex;
		transition: transform 0.2s;
	}

	.filter-toggle .chevron.rotated {
		transform: rotate(180deg);
	}

	.filters-panel {
		display: flex;
		gap: 16px;
		padding: 16px;
		margin-bottom: 16px;
		background: #0a0a0d;
		border: 1px solid #1a1a1f;
		border-radius: 8px;
	}

	.filter-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.filter-group label {
		font-size: 10px;
		font-weight: 500;
		letter-spacing: 0.1em;
		color: #666;
	}

	.filter-group select {
		padding: 8px 12px;
		background: #0c0c0f;
		border: 1px solid #1a1a1f;
		color: #fff;
		font-family: 'Sora', sans-serif;
		font-size: 12px;
		border-radius: 6px;
		cursor: pointer;
		min-width: 140px;
	}

	/* Grid View */
	.grid-view {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
		gap: 12px;
	}

	.device-card {
		display: flex;
		flex-direction: column;
		padding: 16px;
		background: #0c0c0f;
		border: 1px solid #1a1a1f;
		border-radius: 10px;
		cursor: pointer;
		text-align: left;
		transition: all 0.15s;
	}

	.device-card:hover {
		background: #0f0f14;
		border-color: #2a2a30;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 12px;
	}

	.device-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #0a0a0d;
		border-radius: 8px;
		border: 1px solid #1a1a1f;
	}

	.device-icon.online {
		border-color: rgba(0, 212, 255, 0.3);
		background: rgba(0, 212, 255, 0.05);
	}

	.device-status {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #333;
	}

	.device-status.online {
		background: #22c55e;
		box-shadow: 0 0 8px rgba(34, 197, 94, 0.5);
	}

	.card-body {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.device-ip {
		font-family: 'JetBrains Mono', monospace;
		font-size: 14px;
		font-weight: 600;
		color: #fff;
	}

	.device-name {
		font-size: 12px;
		color: #888;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.device-meta {
		font-size: 11px;
		color: #555;
		margin-top: 4px;
	}

	.card-footer {
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid #1a1a1f;
	}

	.device-mac {
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
		color: #444;
	}

	/* List View */
	.list-view {
		background: #0c0c0f;
		border: 1px solid #1a1a1f;
		border-radius: 10px;
		overflow: hidden;
	}

	.list-header {
		display: grid;
		grid-template-columns: 60px 140px 1fr 160px 1fr 100px;
		gap: 12px;
		padding: 12px 16px;
		background: #0a0a0d;
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 0.1em;
		color: #555;
	}

	.list-body {
		max-height: calc(100vh - 340px);
		overflow-y: auto;
	}

	.list-row {
		display: grid;
		grid-template-columns: 60px 140px 1fr 160px 1fr 100px;
		gap: 12px;
		padding: 12px 16px;
		width: 100%;
		border: none;
		background: transparent;
		border-bottom: 1px solid #1a1a1f;
		cursor: pointer;
		text-align: left;
		font-family: inherit;
		color: inherit;
		transition: background 0.1s;
	}

	.list-row:hover {
		background: #0f0f14;
	}

	.list-row:last-child {
		border-bottom: none;
	}

	.status-indicator {
		display: block;
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #333;
	}

	.status-indicator.online {
		background: #22c55e;
		box-shadow: 0 0 6px rgba(34, 197, 94, 0.4);
	}

	.col-ip {
		font-family: 'JetBrains Mono', monospace;
		font-size: 13px;
		color: #fff;
	}

	.col-hostname, .col-vendor {
		font-size: 12px;
		color: #888;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.col-mac {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: #555;
	}

	.col-type {
		font-size: 11px;
		color: #666;
		text-transform: capitalize;
	}

	/* Topology */
	.topology-container {
		height: 500px;
		background: #0c0c0f;
		border: 1px solid #1a1a1f;
		border-radius: 10px;
		overflow: hidden;
	}

	/* Empty & Loading States */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
	}

	.empty-state.full {
		min-height: 400px;
	}

	.empty-icon {
		font-size: 48px;
		color: #1a1a1f;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		font-size: 16px;
		font-weight: 500;
		color: #fff;
	}

	.empty-state p {
		margin: 0 0 24px;
		font-size: 13px;
		color: #555;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		padding: 40px;
		color: #555;
		font-size: 13px;
	}

	.loading-state.full {
		min-height: 400px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.header {
			padding: 0 16px;
		}

		.main {
			padding: 16px;
		}

		.stats-row {
			gap: 16px;
		}

		.stat-value {
			font-size: 22px;
		}

		.list-header {
			display: none;
		}

		.list-row {
			grid-template-columns: 1fr;
			gap: 4px;
		}
	}
</style>
