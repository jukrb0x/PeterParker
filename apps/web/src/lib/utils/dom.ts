import { writable } from 'svelte/store';

export function toggleScroll(lock: boolean) {
	if (typeof document === 'undefined') return;
	document.body.style.overflow = lock ? 'hidden' : '';
}

export function clickOutside(node: HTMLElement, handler: () => void) {
	const onClick = (event: MouseEvent) => {
		if (node && !node.contains(event.target as Node) && !event.defaultPrevented) {
			handler();
		}
	};

	document.addEventListener('click', onClick, true);

	return {
		destroy() {
			document.removeEventListener('click', onClick, true);
		}
	};
}
