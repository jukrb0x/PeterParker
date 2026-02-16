<script lang="ts">
	import type { Device } from '$lib/stores';
	import { DeviceType } from '$lib/scanner/types';
	import { formatDate } from '$lib/utils';
	import { Laptop, Smartphone, Router, Printer, HardDrive, Tv, Gamepad2, HelpCircle, Wifi } from 'lucide-svelte';

	interface Props {
		device: Device;
		onClick?: () => void;
	}

	let { device, onClick }: Props = $props();

	const deviceIcons: Record<DeviceType, typeof Laptop> = {
		[DeviceType.Desktop]: Laptop,
		[DeviceType.Laptop]: Laptop,
		[DeviceType.Mobile]: Smartphone,
		[DeviceType.Tablet]: Smartphone,
		[DeviceType.Router]: Router,
		[DeviceType.Switch]: Router,
		[DeviceType.Printer]: Printer,
		[DeviceType.Nas]: HardDrive,
		[DeviceType.Tv]: Tv,
		[DeviceType.GameConsole]: Gamepad2,
		[DeviceType.Iot]: Wifi,
		[DeviceType.Camera]: Wifi,
		[DeviceType.Unknown]: HelpCircle
	};

	const Icon = deviceIcons[device.deviceType] || HelpCircle;
</script>

<button
	onclick={onClick}
	class="group relative w-full text-left rounded-lg border border-border bg-card p-4 transition-all hover:bg-accent hover:border-primary/50 focus:outline-none focus:ring-2 focus:ring-primary"
>
	<div class="flex items-start gap-4">
		<!-- Icon -->
		<div class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-muted">
			<Icon class="h-6 w-6 text-muted-foreground" />
		</div>

		<!-- Content -->
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-2">
				<h3 class="truncate font-semibold text-foreground">
					{device.hostname || device.vendor || 'Unknown Device'}
				</h3>
				<span
					class="inline-flex h-2 w-2 shrink-0 rounded-full"
					class:bg-pp-online={device.isOnline}
					class:bg-pp-offline={!device.isOnline}
				></span>
			</div>

			<p class="mt-0.5 text-sm text-muted-foreground font-mono">{device.ip}</p>

			{#if device.mac}
				<p class="mt-0.5 text-xs text-muted-foreground font-mono">{device.mac}</p>
			{/if}

			<div class="mt-2 flex items-center gap-3 text-xs text-muted-foreground">
				<span class="capitalize">{device.deviceType}</span>
				<span>•</span>
				<span>{formatDate(new Date(device.lastSeen))}</span>
				{#if device.ports.length > 0}
					<span>•</span>
					<span>{device.ports.length} ports</span>
				{/if}
			</div>
		</div>
	</div>
</button>
