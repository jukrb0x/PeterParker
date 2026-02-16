<script lang="ts">
	import type { ScanProgress } from '$lib/stores';
	import { Progress } from '$lib/components/ui/progress';
	import { Play, Pause, Square } from 'lucide-svelte';

	interface Props {
		progress: ScanProgress;
		percent: number;
		isScanning: boolean;
		isPaused: boolean;
		onPause: () => void;
		onResume: () => void;
		onCancel: () => void;
	}

	let { progress, percent, isScanning, isPaused, onPause, onResume, onCancel }: Props = $props();

	function formatEta(seconds: number | null): string {
		if (!seconds) return 'calculating...';
		if (seconds < 60) return `${seconds}s`;
		if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
		return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`;
	}
</script>

<div class="rounded-lg border border-border bg-card p-4">
	<div class="flex items-center justify-between">
		<div>
			<h3 class="font-semibold text-foreground">
				{#if isScanning}
					Scanning... {progress.currentHost}
				{:else if isPaused}
					Scan Paused
				{:else}
					Scan Complete
				{/if}
			</h3>
			<p class="text-sm text-muted-foreground">
				{progress.scannedHosts} / {progress.totalHosts} hosts • {progress.foundDevices} found
				{#if isScanning && progress.eta}
					• ETA {formatEta(progress.eta)}
				{/if}
			</p>
		</div>

		<div class="flex gap-2">
			{#if isScanning}
				<button
					onclick={onPause}
					class="inline-flex h-8 w-8 items-center justify-center rounded-md bg-secondary hover:bg-secondary/80"
				>
					<Pause class="h-4 w-4" />
				</button>
			{:else if isPaused}
				<button
					onclick={onResume}
					class="inline-flex h-8 w-8 items-center justify-center rounded-md bg-primary text-primary-foreground hover:bg-primary/90"
				>
					<Play class="h-4 w-4" />
				</button>
			{/if}
			<button
				onclick={onCancel}
				class="inline-flex h-8 w-8 items-center justify-center rounded-md bg-destructive text-destructive-foreground hover:bg-destructive/90"
			>
				<Square class="h-4 w-4" />
			</button>
		</div>
	</div>

	<div class="mt-4">
		<Progress value={percent} class="h-2" />
	</div>
</div>
