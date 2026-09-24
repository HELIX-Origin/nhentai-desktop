<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from './Icon.svelte';

	let {
		open,
		title,
		width = 380,
		onclose,
		children,
	}: {
		open: boolean;
		title: string;
		width?: number;
		onclose: () => void;
		children: Snippet;
	} = $props();

	$effect(() => {
		if (!open) return;
		const onKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') onclose();
		};
		window.addEventListener('keydown', onKey);
		const prev = document.documentElement.style.overflow;
		document.documentElement.style.overflow = 'hidden';
		return () => {
			window.removeEventListener('keydown', onKey);
			document.documentElement.style.overflow = prev;
		};
	});
</script>

{#if open}
	<button type="button" class="backdrop" onclick={onclose} aria-label="Close drawer"></button>
	<div class="panel" style={`width:${width}px`} role="dialog" aria-modal="true" aria-label={title}>
		<header class="head">
			<h3>{title}</h3>
			<button class="icon-btn" onclick={onclose} aria-label="Close">
				<Icon name="close" size={16} />
			</button>
		</header>
		<div class="body">
			{@render children()}
		</div>
	</div>
{/if}

<style>
	.backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.55);
		z-index: 40;
		animation: fade 0.16s ease;
	}

	.panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		background: var(--bg-elevated);
		border-left: 1px solid var(--border);
		z-index: 50;
		display: flex;
		flex-direction: column;
		animation: slide 0.18s ease;
		box-shadow: var(--shadow-lg);
	}

	.head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
	}

	h3 {
		margin: 0;
		font-size: 15px;
		font-weight: 650;
	}

	.body {
		flex: 1;
		overflow-y: auto;
		padding: 18px 20px 24px;
	}

	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slide {
		from {
			transform: translateX(28px);
			opacity: 0;
		}
		to {
			transform: none;
			opacity: 1;
		}
	}
</style>