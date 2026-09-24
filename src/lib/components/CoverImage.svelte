<script lang="ts">
	import { proxiedBlobUrl } from '$lib/image';
	import Icon from './Icon.svelte';

	let {
		src,
		alt = '',
		ratio = 0.6667,
		loading: lazy = 'lazy',
		class: klass = '',
	}: {
		src: string;
		alt?: string;
		ratio?: number;
		loading?: 'lazy' | 'eager';
		class?: string;
	} = $props();

	let currentSrc = $state('');
	let placeholder = $state(false);
	let trying = $state(false);

	$effect(() => {
		currentSrc = src;
		placeholder = false;
		trying = false;
	});

	async function onError() {
		if (trying || placeholder) return;
		trying = true;
		try {
			currentSrc = await proxiedBlobUrl(src);
		} catch {
			placeholder = true;
		} finally {
			trying = false;
		}
	}
</script>

<div class={`cover ${klass} ${placeholder ? 'placeholder' : ''}`} style={`aspect-ratio:${ratio}`}>
	{#if placeholder}
		<span class="ph-icon"><Icon name="image" size={22} /></span>
	{:else}
		<img src={currentSrc} alt={alt} loading={lazy} onerror={onError} />
	{/if}
</div>

<style>
	.cover {
		position: relative;
		overflow: hidden;
		background: var(--surface);
		border-radius: var(--radius-sm);
	}

	.cover img {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		display: block;
	}

	.cover.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-faint);
		background: var(--surface);
	}

	.ph-icon {
		opacity: 0.6;
	}
</style>