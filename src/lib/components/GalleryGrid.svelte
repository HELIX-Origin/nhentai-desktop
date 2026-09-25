<script lang="ts">
	import { getSettings } from '$lib/stores/settings.svelte';
	import { matchesBlacklist } from '$lib/stores/blacklist.svelte';
	import type { GalleryListItem } from '$lib/types';
	import GalleryCard from './GalleryCard.svelte';
	import EmptyState from './EmptyState.svelte';

	let { galleries }: { galleries: GalleryListItem[] } = $props();

	const s = getSettings();

	const showBlur = $derived(s.blacklistEnabled && s.blacklistMode === 'blur');
	const hideBlocked = $derived(s.blacklistEnabled && s.blacklistMode === 'hide');

	const visible = $derived(
		hideBlocked ? galleries.filter((g) => !matchesBlacklist(g)) : galleries,
	);
</script>

{#if visible.length === 0}
	<EmptyState
		icon="shield"
		title="No galleries to show"
		description="Everything here is filtered by your active blacklist."
	/>
{:else}
	<div class="grid" class:compact={s.density === 'compact'} data-blur={showBlur}>
		{#each visible as g}
			<GalleryCard gallery={g} />
		{/each}
	</div>
{/if}

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(176px, 1fr));
		gap: 12px;
		padding: 4px;
	}

	.grid.compact {
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 8px;
	}

	.grid :global(.card) {
		width: 100%;
	}
</style>