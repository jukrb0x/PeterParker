<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { devicesStore } from '$lib/stores';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft, Download, Trash2, RotateCw } from 'lucide-svelte';
	import { formatDate } from '$lib/utils';

	let ip = $derived($page.params.ip ?? '');
	let device = $derived(devicesStore.getByIp(ip));

	function goBack() {
		goto('/');
	}
</script>

{#if device}
	<div class="min-h-screen bg-pp-bg">
		<!-- Header -->
		<header class="sticky top-0 z-50 border-b border-border bg-pp-surface/95 backdrop-blur">
			<div class="flex h-14 items-center justify-between px-4 lg:px-6">
				<Button variant="ghost" size="sm" onclick={goBack}>
					<ArrowLeft class="mr-2 h-4 w-4" />
					Back
				</Button>
				<div class="flex gap-2">
					<Button variant="ghost" size="icon">
						<Download class="h-4 w-4" />
					</Button>
					<Button variant="ghost" size="icon">
						<RotateCw class="h-4 w-4" />
					</Button>
					<Button variant="ghost" size="icon" class="text-destructive">
						<Trash2 class="h-4 w-4" />
					</Button>
				</div>
			</div>
		</header>

		<main class="p-4 lg:p-6">
			<!-- Device Header -->
			<div class="flex items-start justify-between">
				<div>
					<p class="text-sm text-muted-foreground font-mono">{device.ip}</p>
					<h1 class="mt-1 text-2xl font-bold">
						{device.hostname || device.vendor || 'Unknown Device'}
					</h1>
					<div class="mt-2 flex items-center gap-2">
						<span
							class="inline-flex h-2 w-2 rounded-full"
							class:bg-pp-online={device.isOnline}
							class:bg-pp-offline={!device.isOnline}
						></span>
						<span class="text-sm text-muted-foreground">
							{device.isOnline ? 'Online' : 'Offline'}
						</span>
					</div>
				</div>
			</div>

			<!-- Info Grid -->
			<div class="mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<div class="rounded-lg border border-border bg-card p-4">
					<p class="text-sm text-muted-foreground">MAC Address</p>
					<p class="mt-1 font-mono text-sm">{device.mac || 'Unknown'}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-4">
					<p class="text-sm text-muted-foreground">Vendor</p>
					<p class="mt-1">{device.vendor || 'Unknown'}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-4">
					<p class="text-sm text-muted-foreground">Device Type</p>
					<p class="mt-1 capitalize">{device.deviceType}</p>
				</div>
				<div class="rounded-lg border border-border bg-card p-4">
					<p class="text-sm text-muted-foreground">OS</p>
					<p class="mt-1">{device.os?.name || 'Unknown'}</p>
				</div>
			</div>

			<!-- Open Ports -->
			<div class="mt-6 rounded-lg border border-border bg-card">
				<div class="flex items-center justify-between border-b border-border p-4">
					<h2 class="font-semibold">Open Ports ({device.ports.length})</h2>
					<Button variant="outline" size="sm">
						<RotateCw class="mr-2 h-4 w-4" />
						Rescan
					</Button>
				</div>
				{#if device.ports.length > 0}
					<table class="w-full text-sm">
						<thead>
							<tr class="border-b border-border text-left">
								<th class="p-4 font-medium">Port</th>
								<th class="p-4 font-medium">Service</th>
								<th class="p-4 font-medium">Version</th>
								<th class="p-4 font-medium">Banner</th>
							</tr>
						</thead>
						<tbody>
							{#each device.ports as port}
								<tr class="border-b border-border/50">
									<td class="p-4 font-mono">{port.number}/{port.protocol}</td>
									<td class="p-4">{port.service?.name || '-'}</td>
									<td class="p-4">{port.service?.version || '-'}</td>
									<td class="p-4 truncate max-w-xs">{port.banner || '-'}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				{:else}
					<div class="p-8 text-center text-muted-foreground">
						No open ports detected
					</div>
				{/if}
			</div>
		</main>
	</div>
{:else}
	<div class="flex h-screen items-center justify-center">
		<div class="text-center">
			<p class="text-muted-foreground">Device not found</p>
			<Button variant="link" onclick={goBack}>Go back</Button>
		</div>
	</div>
{/if}
