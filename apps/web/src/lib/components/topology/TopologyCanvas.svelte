<script lang="ts">
	import { onMount } from 'svelte';
	import { untrack } from 'svelte';
	import * as d3 from 'd3';
	import type { Device } from '$lib/scanner/types';
	import { topologyStore } from '$lib/topology/store.svelte';
	import { GATEWAY_NODE_SIZE, DEVICE_NODE_SIZE, COLORS } from '$lib/topology/types';

	interface Props {
		devices: Device[];
		onNodeClick?: (deviceId: string) => void;
		onNodeDoubleClick?: (deviceId: string) => void;
	}

	let { devices, onNodeClick, onNodeDoubleClick }: Props = $props();

	let container: HTMLDivElement;
	let svg: SVGSVGElement;
	let width = $state(800);
	let height = $state(500);

	// Pan state
	let isPanning = $state(false);
	let panStart = $state({ x: 0, y: 0 });

	// D3 simulation
	let simulation: d3.Simulation<any, undefined> | null = null;

	// Track previous device IDs to avoid unnecessary re-renders
	let lastDeviceIds: string = '';

	// Watch for device changes
	$effect(() => {
		const currentIds = devices.map(d => d.id).sort().join(',');
		if (devices.length > 0 && width > 0 && height > 0 && currentIds !== lastDeviceIds) {
			lastDeviceIds = currentIds;
			topologyStore.setDevices(devices, width, height);
			initSimulation();
		}
	});

	onMount(() => {
		updateDimensions();
		window.addEventListener('resize', updateDimensions);
		return () => {
			window.removeEventListener('resize', updateDimensions);
			if (simulation) simulation.stop();
		};
	});

	function updateDimensions() {
		if (container) {
			width = container.clientWidth;
			height = container.clientHeight;
		}
	}

	function initSimulation() {
		if (simulation) simulation.stop();

		const nodes = topologyStore.state.nodes.map(n => ({ ...n }));

		simulation = d3.forceSimulation(nodes)
			.force('link', d3.forceLink()
				.id((d: any) => d.id)
				.distance(100)
				.strength(0.3)
			)
			.force('charge', d3.forceManyBody().strength(-150))
			.force('center', d3.forceCenter(width / 2, height / 2))
			.force('collision', d3.forceCollide().radius(50))
			.alphaDecay(0.02)
			.on('tick', () => {
				nodes.forEach((simNode: any) => {
					const storeNode = topologyStore.state.nodes.find(n => n.id === simNode.id);
					if (storeNode) {
						// Clamp positions to canvas bounds
						storeNode.x = Math.max(30, Math.min(width - 30, simNode.x));
						storeNode.y = Math.max(30, Math.min(height - 30, simNode.y));
					}
				});
			});

		(simulation as any).force('link')?.links(topologyStore.state.edges);
	}

	function handleNodeClick(event: MouseEvent, nodeId: string) {
		event.stopPropagation();
		topologyStore.selectNode(nodeId);
		onNodeClick?.(nodeId);
	}

	function handleNodeDoubleClick(event: MouseEvent, nodeId: string) {
		event.stopPropagation();
		onNodeDoubleClick?.(nodeId);
	}

	function handleNodeKeydown(event: KeyboardEvent, nodeId: string) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			topologyStore.selectNode(nodeId);
			onNodeClick?.(nodeId);
		}
	}

	// Canvas panning
	function handleMouseDown(e: MouseEvent) {
		if (e.target === svg || (e.target as Element).tagName === 'svg') {
			isPanning = true;
			panStart = { x: e.clientX - topologyStore.state.pan.x, y: e.clientY - topologyStore.state.pan.y };
		}
	}

	function handleMouseMove(e: MouseEvent) {
		if (isPanning) {
			topologyStore.setPan(e.clientX - panStart.x, e.clientY - panStart.y);
		}
	}

	function handleMouseUp() {
		isPanning = false;
	}

	function handleWheel(e: WheelEvent) {
		e.preventDefault();
		const delta = e.deltaY > 0 ? 0.9 : 1.1;
		topologyStore.setZoom(topologyStore.state.zoom * delta);
	}

	function getNodeColor(node: typeof topologyStore.state.nodes[0]): string {
		if (!node.device.isOnline) return COLORS.offline;
		return COLORS.online;
	}
</script>

<div
	bind:this={container}
	class="relative w-full h-full overflow-hidden cursor-grab"
	class:cursor-grabbing={isPanning}
	role="application"
	aria-label="Network topology visualization. Use mouse to pan and scroll to zoom."
	onmousedown={handleMouseDown}
	onmousemove={handleMouseMove}
	onmouseup={handleMouseUp}
	onmouseleave={handleMouseUp}
	onwheel={handleWheel}
>
	<svg
		bind:this={svg}
		{width}
		{height}
		class="block"
	>
		<g transform="translate({topologyStore.state.pan.x}, {topologyStore.state.pan.y}) scale({topologyStore.state.zoom})">
			<!-- Edges -->
			{#each topologyStore.state.edges as edge (edge.id)}
				{@const source = topologyStore.state.nodes.find(n => n.id === edge.source)}
				{@const target = topologyStore.state.nodes.find(n => n.id === edge.target)}
				{#if source && target}
					<line
						x1={source.x}
						y1={source.y}
						x2={target.x}
						y2={target.y}
						stroke={COLORS.edge}
						stroke-width="1"
					/>
				{/if}
			{/each}

			<!-- Nodes -->
			{#each topologyStore.state.nodes as node (node.id)}
				{@const size = node.isGateway ? GATEWAY_NODE_SIZE : DEVICE_NODE_SIZE}
				{@const color = getNodeColor(node)}
				{@const isSelected = topologyStore.state.selectedNodeId === node.id}
				<g
					transform="translate({node.x}, {node.y})"
					class="cursor-pointer"
					role="button"
					tabindex="0"
					aria-label="{node.device.hostname || node.device.vendor || node.device.deviceType} - {node.device.ip}"
					onclick={(e) => handleNodeClick(e, node.id)}
					ondblclick={(e) => handleNodeDoubleClick(e, node.id)}
					onkeydown={(e) => handleNodeKeydown(e, node.id)}
					onmouseenter={() => topologyStore.hoverNode(node.id)}
					onmouseleave={() => topologyStore.hoverNode(null)}
				>
					<!-- Glow for online -->
					{#if node.device.isOnline}
						<circle r={size / 2 + 6} fill={COLORS.onlineGlow} opacity="0.5" />
					{/if}

					<!-- Selection ring -->
					{#if isSelected}
						<circle r={size / 2 + 3} fill="none" stroke={COLORS.selected} stroke-width="2" />
					{/if}

					<!-- Node circle -->
					<circle
						r={size / 2}
						fill="#1a1a1a"
						stroke={color}
						stroke-width="2"
					/>

					<!-- IP label -->
					<text
						y={size / 2 + 16}
						text-anchor="middle"
						fill="#888"
						font-size="11"
						font-family="monospace"
					>
						{node.device.ip}
					</text>

					<!-- Device type label -->
					<text
						y={-size / 2 - 8}
						text-anchor="middle"
						fill="#fff"
						font-size="10"
					>
						{node.device.hostname || node.device.vendor || node.device.deviceType}
					</text>

					<!-- Status indicator -->
					<circle
						cx={size / 2 - 4}
						cy={-size / 2 + 4}
						r="4"
						fill={node.device.isOnline ? '#22c55e' : '#666'}
					/>

					<!-- New badge -->
					{#if node.isNew}
						<circle cx={-size / 2 + 4} cy={-size / 2 + 4} r="6" fill="#22c55e" />
						<text x={-size / 2 + 4} y={-size / 2 + 5} text-anchor="middle" fill="white" font-size="7" font-weight="bold">N</text>
					{/if}
				</g>
			{/each}
		</g>
	</svg>

	<!-- Tooltip -->
	{#if topologyStore.hoveredNode}
		{@const node = topologyStore.hoveredNode}
		<div class="absolute top-4 left-4 bg-popover border border-border rounded-lg p-3 shadow-lg pointer-events-none">
			<div class="flex items-center justify-between gap-4">
				<span class="font-mono text-sm">{node.device.ip}</span>
				<span class="text-xs {node.device.isOnline ? 'text-green-500' : 'text-muted-foreground'}">
					{node.device.isOnline ? 'Online' : 'Offline'}
				</span>
			</div>
			{#if node.device.hostname}
				<div class="mt-1 text-sm font-medium">{node.device.hostname}</div>
			{/if}
			{#if node.device.vendor}
				<div class="text-xs text-muted-foreground">{node.device.vendor}</div>
			{/if}
		</div>
	{/if}

	<!-- Zoom indicator -->
	<div class="absolute bottom-4 right-4 text-xs text-muted-foreground bg-popboard/80 px-2 py-1 rounded">
		{Math.round(topologyStore.state.zoom * 100)}%
	</div>
</div>
