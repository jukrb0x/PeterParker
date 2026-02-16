import { formatDate } from '$lib/utils';

/**
 * Format bytes to human readable
 */
export function formatBytes(bytes: number): string {
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
	if (bytes === 0) return '0 B';
	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
}

/**
 * Format duration in ms to human readable
 */
export function formatDuration(ms: number): string {
	if (ms < 1000) return `${ms}ms`;
	if (ms < 60000) return `${Math.floor(ms / 1000)}s`;
	if (ms < 3600000) return `${Math.floor(ms / 60000)}m`;
	return `${Math.floor(ms / 3600000)}h ${Math.floor((ms % 3600000) / 60000)}m`;
}

/**
 * Truncate text with ellipsis
 */
export function truncate(text: string, maxLength: number): string {
	if (text.length <= maxLength) return text;
	return text.slice(0, maxLength - 3) + '...';
}

/**
 * Convert CIDR to range
 */
export function cidrToRange(cidr: string): { start: string; end: string; count: number } {
	const [ip, prefix] = cidr.split('/');
	const prefixNum = parseInt(prefix, 10);
	const count = Math.pow(2, 32 - prefixNum);
	
	const parts = ip.split('.').map(Number);
	const base = parts.reduce((acc, part, i) => acc + (part << (24 - i * 8)), 0);
	
	const startIp = base & (~(count - 1));
	const endIp = startIp + count - 1;
	
	const toIp = (n: number) => [
		(n >>> 24) & 255,
		(n >>> 16) & 255,
		(n >>> 8) & 255,
		n & 255
	].join('.');
	
	return {
		start: toIp(startIp + 1),
		end: toIp(endIp - 1),
		count: count - 2
	};
}
