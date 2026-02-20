<script lang="ts">
	import { onMount } from 'svelte';
	import * as d3 from 'd3';
	import type { Device, DeviceType } from '$lib/scanner/types';
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

	let isPanning = $state(false);
	let panStart = $state({ x: 0, y: 0 });

	let simulation: d3.Simulation<any, undefined> | null = null;
	let lastDeviceIds: string = '';

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
		const edges = topologyStore.state.edges.map(e => ({ ...e }));

		simulation = d3.forceSimulation(nodes)
			.force('link', d3.forceLink(edges)
				.id((d: any) => d.id)
				.distance(120)
				.strength(0.4)
			)
			.force('charge', d3.forceManyBody().strength(-200))
			.force('center', d3.forceCenter(width / 2, height / 2))
			.force('collision', d3.forceCollide().radius(60))
			.alphaDecay(0.02)
			.on('tick', () => {
				nodes.forEach((simNode: any) => {
					const storeNode = topologyStore.state.nodes.find(n => n.id === simNode.id);
					if (storeNode) {
						storeNode.x = Math.max(40, Math.min(width - 40, simNode.x));
						storeNode.y = Math.max(40, Math.min(height - 40, simNode.y));
					}
				});
				// Update edge positions based on node positions
				edges.forEach((simEdge: any) => {
					const storeEdge = topologyStore.state.edges.find(e => e.id === simEdge.id);
					if (storeEdge && simEdge.source && simEdge.target) {
						// D3 replaces source/target with node references
					}
				});
			});

		(simulation as any).force('link')?.links(edges);
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

	// SVG icon paths (same as DeviceIcon component)
	const deviceIcons: Record<string, string> = {
		router: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z',
		desktop: 'M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7l-2 3v1h8v-1l-2-3h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 12H3V4h18v10z',
		laptop: 'M20 18c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z',
		mobile: 'M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z',
		tablet: 'M19 1H5c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm-7 20c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm7-3H5V4h14v14z',
		iot: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm-2-3.5l4-2.5-4-2.5v5zm3-8c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm-4 0c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1z',
		printer: 'M19 8H5c-1.66 0-3 1.34-3 3v6h4v4h12v-4h4v-6c0-1.66-1.34-3-3-3zm-3 11H8v-5h8v5zm3-7c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-1-9H6v4h12V3z',
		nas: 'M4 6h16v2H4zm0 5h16v2H4zm0 5h16v2H4zM2 4h2v16H2zm18 0h2v16h-2z',
		camera: 'M17 10.5V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4z',
		tv: 'M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12z',
		game_console: 'M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-10 7H8v3H6v-3H3v-2h3V8h2v3h3v2zm4.5 2c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4-3c-.83 0-1.5-.67-1.5-1.5S18.67 9 19.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z',
		server: 'M2 2h20v6H2zm2 2v2h16V4zm-2 7h20v6H2zm2 2v2h16v-2zm-2 7h20v6H2zm2 2v2h16v-2z',
		switch: 'M20 7H4c-1.1 0-2 .9-2 2v6c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zM9 12H5v-2h4v2zm6 0h-4v-2h4v2zm4 0h-2v-2h2v2z',
		unknown: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z'
	};

	function getDeviceIconPath(deviceType: DeviceType | string): string {
		const type = deviceType?.toLowerCase?.() ?? 'unknown';
		return deviceIcons[type] || deviceIcons.unknown;
	}
</script>

<div
	bind:this={container}
	class="relative w-full h-full overflow-hidden cursor-grab bg-[#0a0a0d]"
	class:cursor-grabbing={isPanning}
	role="application"
	aria-label="Network topology visualization. Use mouse to pan and scroll to zoom."
	onmousedown={handleMouseDown}
	onmousemove={handleMouseMove}
	onmouseup={handleMouseUp}
	onmouseleave={handleMouseUp}
	onwheel={handleWheel}
>
	<svg bind:this={svg} {width} {height} class="block">
		<defs>
			<!-- Gradient for edges -->
			<linearGradient id="edgeGradient" x1="0%" y1="0%" x2="100%" y2="0%">
				<stop offset="0%" stop-color="#00d4ff" stop-opacity="0.6" />
				<stop offset="50%" stop-color="#00d4ff" stop-opacity="0.3" />
				<stop offset="100%" stop-color="#00d4ff" stop-opacity="0.6" />
			</linearGradient>

			<!-- Glow filter for online nodes -->
			<filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
				<feGaussianBlur stdDeviation="4" result="coloredBlur" />
				<feMerge>
					<feMergeNode in="coloredBlur" />
					<feMergeNode in="SourceGraphic" />
				</feMerge>
			</filter>

			<!-- Drop shadow for nodes -->
			<filter id="shadow" x="-50%" y="-50%" width="200%" height="200%">
				<feDropShadow dx="0" dy="2" stdDeviation="3" flood-color="#000" flood-opacity="0.5" />
			</filter>
		</defs>

		<g transform="translate({topologyStore.state.pan.x}, {topologyStore.state.pan.y}) scale({topologyStore.state.zoom})">
			<!-- Edges (connections) -->
			{#each topologyStore.state.edges as edge (edge.id)}
				{@const source = topologyStore.state.nodes.find(n => n.id === edge.source)}
				{@const target = topologyStore.state.nodes.find(n => n.id === edge.target)}
				{#if source && target}
					<line
						x1={source.x}
						y1={source.y}
						x2={target.x}
						y2={target.y}
						stroke="url(#edgeGradient)"
						stroke-width="2"
						stroke-linecap="round"
					/>
					<!-- Animated pulse on edge -->
					<line
						x1={source.x}
						y1={source.y}
						x2={target.x}
						y2={target.y}
						stroke="#00d4ff"
						stroke-width="1"
						stroke-opacity="0.4"
						stroke-dasharray="4 8"
						class="edge-pulse"
					/>
				{/if}
			{/each}

			<!-- Nodes -->
			{#each topologyStore.state.nodes as node (node.id)}
				{@const size = node.isGateway ? GATEWAY_NODE_SIZE : DEVICE_NODE_SIZE}
				{@const color = getNodeColor(node)}
				{@const isSelected = topologyStore.state.selectedNodeId === node.id}
				{@const iconPath = getDeviceIconPath(node.device.deviceType)}
				<g
					transform="translate({node.x}, {node.y})"
					class="cursor-pointer node-group"
					role="button"
					tabindex="0"
					aria-label="{node.device.hostname || node.device.vendor || node.device.deviceType} - {node.device.ip}"
					onclick={(e) => handleNodeClick(e, node.id)}
					ondblclick={(e) => handleNodeDoubleClick(e, node.id)}
					onkeydown={(e) => handleNodeKeydown(e, node.id)}
					onmouseenter={() => topologyStore.hoverNode(node.id)}
					onmouseleave={() => topologyStore.hoverNode(null)}
				>
					<!-- Outer glow for online devices -->
					{#if node.device.isOnline}
						<circle r={size / 2 + 8} fill="#00d4ff" opacity="0.15" />
					{/if}

					<!-- Selection ring -->
					{#if isSelected}
						<circle r={size / 2 + 4} fill="none" stroke="#00d4ff" stroke-width="3" />
					{/if}

					<!-- Node background -->
					<circle
						r={size / 2}
						fill="#151518"
						stroke={color}
						stroke-width="2"
						filter="url(#shadow)"
					/>

					<!-- Device icon inside the node -->
					<g transform="translate(-{size / 4}, -{size / 4})">
						<path
							d={iconPath}
							fill={node.device.isOnline ? '#00d4ff' : '#555'}
							transform="scale({size / 24})"
						/>
					</g>

					<!-- Online status indicator -->
					<circle
						cx={size / 2 - 6}
						cy={-size / 2 + 6}
						r="5"
						fill={node.device.isOnline ? '#22c55e' : '#444'}
						stroke="#151518"
						stroke-width="2"
					/>

					<!-- Label: IP address -->
					<text
						y={size / 2 + 18}
						text-anchor="middle"
						fill="#888"
						font-size="11"
						font-family="'JetBrains Mono', monospace"
					>
						{node.device.ip}
					</text>

					<!-- Label: Hostname/Type above -->
					<text
						y={-size / 2 - 10}
						text-anchor="middle"
						fill="#ccc"
						font-size="11"
						font-weight="500"
					>
						{node.device.hostname || node.device.vendor || node.device.deviceType}
					</text>

					<!-- Gateway badge -->
					{#if node.isGateway}
						<rect
							x="-18"
							y={size / 2 + 26}
							width="36"
							height="14"
							rx="3"
							fill="#00d4ff"
						/>
						<text
							y={size / 2 + 36}
							text-anchor="middle"
							fill="#000"
							font-size="9"
							font-weight="600"
						>
							GATEWAY
						</text>
					{/if}
				</g>
			{/each}
		</g>
	</svg>

	<!-- Hover tooltip -->
	{#if topologyStore.hoveredNode}
		{@const node = topologyStore.hoveredNode}
		<div class="tooltip">
			<div class="tooltip-header">
				<span class="tooltip-ip">{node.device.ip}</span>
				<span class="tooltip-status" class:online={node.device.isOnline}>
					{node.device.isOnline ? 'Online' : 'Offline'}
				</span>
			</div>
			{#if node.device.hostname}
				<div class="tooltip-hostname">{node.device.hostname}</div>
			{/if}
			<div class="tooltip-info">
				<span>{node.device.vendor || 'Unknown vendor'}</span>
				<span class="tooltip-divider">•</span>
				<span>{node.device.deviceType}</span>
				{#if node.device.ports.length > 0}
					<span class="tooltip-divider">•</span>
					<span>{node.device.ports.length} ports</span>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Zoom indicator -->
	<div class="zoom-indicator">
		{Math.round(topologyStore.state.zoom * 100)}%
	</div>

	<!-- Legend -->
	<div class="legend">
		<div class="legend-item">
			<span class="legend-dot online"></span>
			<span>Online</span>
		</div>
		<div class="legend-item">
			<span class="legend-dot offline"></span>
			<span>Offline</span>
		</div>
	</div>
</div>

<style>
	.edge-pulse {
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { stroke-opacity: 0.2; }
		50% { stroke-opacity: 0.6; }
	}

	.tooltip {
		position: absolute;
		top: 16px;
		left: 16px;
		background: #151518;
		border: 1px solid #2a2a30;
		border-radius: 8px;
		padding: 12px;
		min-width: 180px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
		pointer-events: none;
	}

	.tooltip-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 4px;
	}

	.tooltip-ip {
		font-family: 'JetBrains Mono', monospace;
		font-size: 13px;
		font-weight: 600;
		color: #fff;
	}

	.tooltip-status {
		font-size: 11px;
		font-weight: 500;
		color: #666;
		padding: 2px 6px;
		border-radius: 4px;
		background: #222;
	}

	.tooltip-status.online {
		color: #22c55e;
		background: rgba(34, 197, 94, 0.15);
	}

	.tooltip-hostname {
		font-size: 12px;
		font-weight: 500;
		color: #ccc;
		margin-bottom: 4px;
	}

	.tooltip-info {
		font-size: 11px;
		color: #888;
	}

	.tooltip-divider {
		margin: 0 4px;
		color: #444;
	}

	.zoom-indicator {
		position: absolute;
		bottom: 16px;
		right: 16px;
		font-size: 11px;
		font-family: 'JetBrains Mono', monospace;
		color: #666;
		background: #151518;
		padding: 4px 8px;
		border-radius: 4px;
		border: 1px solid #2a2a30;
	}

	.legend {
		position: absolute;
		bottom: 16px;
		left: 16px;
		display: flex;
		gap: 16px;
		background: #151518;
		padding: 8px 12px;
		border-radius: 6px;
		border: 1px solid #2a2a30;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 11px;
		color: #888;
	}

	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.legend-dot.online {
		background: #00d4ff;
		box-shadow: 0 0 6px rgba(0, 212, 255, 0.5);
	}

	.legend-dot.offline {
		background: #444;
	}
</style>
