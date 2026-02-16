import { describe, it, expect } from 'vitest';
import { 
	formatBytes, 
	formatDuration, 
	truncate, 
	cidrToRange 
} from '$lib/utils/format';

describe('format utilities', () => {
	describe('formatBytes', () => {
		it('formats bytes', () => {
			expect(formatBytes(0)).toBe('0 B');
			expect(formatBytes(1024)).toBe('1.0 KB');
			expect(formatBytes(1024 * 1024)).toBe('1.0 MB');
			expect(formatBytes(1024 * 1024 * 1024)).toBe('1.0 GB');
		});
	});

	describe('formatDuration', () => {
		it('formats milliseconds', () => {
			expect(formatDuration(500)).toBe('500ms');
			expect(formatDuration(5000)).toBe('5s');
			expect(formatDuration(120000)).toBe('2m');
			expect(formatDuration(3600000)).toBe('1h 0m');
		});
	});

	describe('truncate', () => {
		it('truncates long strings', () => {
			expect(truncate('hello world', 8)).toBe('hello...');
			expect(truncate('hello', 10)).toBe('hello');
		});
	});

	describe('cidrToRange', () => {
		it('converts CIDR to IP range', () => {
			const result = cidrToRange('192.168.1.0/24');
			expect(result.start).toBe('192.168.1.1');
			expect(result.end).toBe('192.168.1.254');
			expect(result.count).toBe(254);
		});
	});
});
