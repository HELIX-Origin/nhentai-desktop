<script lang="ts">
	import { pagePath, proxiedBlobUrl } from '$lib/image';
	import type { PageInfo } from '$lib/types';
	import Icon from './Icon.svelte';

	let {
		page,
		fit = 'width',
		visible = true,
	}: {
		page: PageInfo;
		fit?: 'width' | 'height' | 'contain';
		visible?: boolean;
	} = $props();

	let currentSrc = $state('');
	let failed = $state(false);
	let trying = $state(false);

	$effect(() => {
		currentSrc = pagePath(page.path);
		failed = false;
		trying = false;
	});

	async function onError() {
		if (trying || failed) return;
		trying = true;
		try {
			currentSrc = await proxiedBlobUrl(pagePath(page.path));
			failed = false;
		} catch {
			failed = true;
		} finally {
			trying = false;
		}
	}
</script>

<div class="slot" class:off={!visible} data-fit={fit}>
	{#if failed}
		<span class="ph"><Icon name="image" size={26} /></span>
	{:else}
		<img src={currentSrc} alt={`Page ${page.number}`} draggable="false" onerror={onError} />
	{/if}
</div>

<style>
	.slot {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		max-width: 100%;
		max-height: 100%;
	}

	.slot[data-fit='width'] {
		width: 100%;
		max-height: none;
	}

	.slot[data-fit='height'] {
		height: 100%;
		max-width: none;
	}

	.slot[data-fit='contain'] {
		width: 100%;
		height: 100%;
	}

	.slot.off {
		visibility: hidden;
	}

	.slot img {
		display: block;
		user-select: none;
	}

	.slot[data-fit='width'] img {
		width: 100%;
		height: auto;
	}

	.slot[data-fit='height'] img {
		height: 100%;
		width: auto;
	}

	.slot[data-fit='contain'] img {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.ph {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 120px;
		height: 170px;
		border-radius: var(--radius);
		background: var(--surface);
		color: var(--text-faint);
	}
</style>