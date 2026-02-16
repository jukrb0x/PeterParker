<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { devicesStore, scanStore, settingsStore } from '$lib/stores';
	import { DeviceCard } from '$lib/components/device';
	import { ScanProgressPanel } from '$lib/components/scan';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Wifi, Search, Grid, List, Settings, Plus } from 'lucide-svelte';

	// Local state - directly bind to store filters
	let query = $state(devicesStore.filters.query);
	let type = $state(devicesStore.filters.type);
	let status = $state(devicesStore.filters.status);
	let viewMode = $state(settingsStore.settings.ui.viewMode);

	// Derived from stores
	let devices = $derived(devicesStore.filteredDevices);
	let stats = $derived(devicesStore.stats);
	let isScanning = $derived(scanStore.isScanning);

	// Update store when local state changes (using untrack to prevent loops)
	function updateFilters() {
		devicesStore.setFilters({ query, type, status });
	}

	onMount(() => {
		devicesStore.load();
	});

	function startScan() {
		scanStore.start(settingsStore.settings.scanDefaults);
	}

	function toggleViewMode() {
		viewMode = viewMode === 'grid' ? 'list' : 'grid';
		settingsStore.updateUI({ viewMode });
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

<div class="min-h-screen bg-pp-bg">
	<!-- Header -->
	<header class="sticky top-0 z-50 border-b border-border bg-pp-surface/95 backdrop-blur supports-[backdrop-filter]:bg-pp-surface/60">
		<div class="flex h-14 items-center justify-between px-4 lg:px-6">
			<div class="flex items-center gap-3">
				<Wifi class="h-6 w-6 text-primary" />
				<h1 class="text-lg font-semibold tracking-tight">PeterParker</h1>
			</div>
			<div class="flex items-center gap-2">
				<Button variant="ghost" size="icon" onclick={toggleViewMode}>
					{#if viewMode === 'grid'}
						<List class="h-4 w-4" />
					{:else}
						<Grid class="h-4 w-4" />
					{/if}
				</Button>
				<Button variant="ghost" size="icon">
					<Settings class="h-4 w-4" />
				</Button>
				<Button onclick={startScan} disabled={isScanning}>
					<Plus class="mr-2 h-4 w-4" />
					Scan
				</Button>
			</div>
		</div>
	</header>

	<main class="p-4 lg:p-6">
		<!-- Stats Cards -->
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
			<div class="rounded-lg border border-border bg-card p-4">
				<p class="text-sm text-muted-foreground">Total Devices</p>
				<p class="mt-1 text-2xl font-bold">{stats.total}</p>
			</div>
			<div class="rounded-lg border border-border bg-card p-4">
				<p class="text-sm text-muted-foreground">Online</p>
				<p class="mt-1 text-2xl font-bold text-pp-success">{stats.online}</p>
			</div>
			<div class="rounded-lg border border-border bg-card p-4">
				<p class="text-sm text-muted-foreground">Offline</p>
				<p class="mt-1 text-2xl font-bold text-pp-offline">{stats.offline}</p>
			</div>
			<div class="rounded-lg border border-border bg-card p-4">
				<p class="text-sm text-muted-foreground">New Today</p>
				<p class="mt-1 text-2xl font-bold">0</p>
			</div>
		</div>

		<!-- Scan Progress -->
		{#if scanStore.progress}
			<div class="mt-6">
				<ScanProgressPanel
					progress={scanStore.progress}
					percent={scanStore.progressPercent}
					isScanning={scanStore.isScanning}
					isPaused={scanStore.isPaused}
					onPause={() => scanStore.pause()}
					onResume={() => scanStore.resume()}
					onCancel={() => scanStore.cancel()}
				/>
			</div>
		{/if}

		<!-- Filters -->
		<div class="mt-6 flex flex-col gap-4 sm:flex-row">
			<div class="relative flex-1">
				<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
				<Input
					placeholder="Search IP, MAC, hostname..."
					class="pl-9"
					bind:value={query}
					oninput={updateFilters}
				/>
			</div>
			<select
				bind:value={type}
				onchange={updateFilters}
				class="h-9 rounded-md border border-input bg-background px-3 text-sm"
			>
				{#each deviceTypes as t}
					<option value={t.value}>{t.label}</option>
				{/each}
			</select>
			<select
				bind:value={status}
				onchange={updateFilters}
				class="h-9 rounded-md border border-input bg-background px-3 text-sm"
			>
				<option value="all">All Status</option>
				<option value="online">Online</option>
				<option value="offline">Offline</option>
			</select>
		</div>

		<!-- Device Grid -->
		<div
			class="mt-6 grid gap-4"
			class:sm:grid-cols-2={viewMode === 'grid'}
			class:lg:grid-cols-3={viewMode === 'grid'}
		>
			{#each devices as device (device.id)}
				<DeviceCard
					{device}
					onClick={() => goto(`/device/${encodeURIComponent(device.ip)}`)}
				/>
			{:else}
				<div class="col-span-full flex h-64 items-center justify-center rounded-lg border border-dashed border-border">
					<div class="text-center">
						<Wifi class="mx-auto h-8 w-8 text-muted-foreground" />
						<p class="mt-2 text-sm text-muted-foreground">No devices found</p>
						{#if devicesStore.isLoading}
							<p class="text-xs text-muted-foreground">Loading...</p>
						{:else}
							<Button variant="link" onclick={startScan} class="mt-2">
								Start a scan
							</Button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</main>
</div>
