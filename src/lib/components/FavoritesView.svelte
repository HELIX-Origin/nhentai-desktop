<script lang="ts">
	import { getFavorites, clearFavorites, removeFavorite } from '$lib/stores/library.svelte';
	import { titlebarQuery } from '$lib/stores/titlebarSearch.svelte';
	import type { GalleryListItem } from '$lib/types';
	import { formatCount } from '$lib/format';
	import { thumbPath } from '$lib/image';
	import Icon from './Icon.svelte';
	import CoverImage from './CoverImage.svelte';
	import EmptyState from './EmptyState.svelte';

	const query = $derived(titlebarQuery.value.trim().toLowerCase());
	const sorted = $derived(
		getFavorites()
			.filter((g) => {
				if (!query) return true;
				const text = `${g.english_title} ${g.japanese_title ?? ''}`.toLowerCase();
				return text.includes(query);
			})
			.sort((a, b) => b.id - a.id),
	);

	function onRemove(id: number) {
		removeFavorite(id);
	}
</script>

<div class="favorites">
	<div class="head">
		<h2>Favorites</h2>
		{#if sorted.length > 0}
			<button class="btn btn-ghost faint" onclick={clearFavorites}>
				<Icon name="close" size={14} />
				Clear all
			</button>
		{/if}
	</div>

	{#if sorted.length === 0}
		<EmptyState
			icon="heart"
			title="No favorites yet"
			description="Tap the heart on any gallery to save it here — stored locally on this device."
		/>
	{:else}
		<div class="grid">
			{#each sorted as g (g.id)}
				<div class="cell">
					<a class="card" href={`/gallery/${g.id}`}>
						<CoverImage src={thumbPath(g.thumbnail)} alt={g.english_title} />
						<div class="info">
							<span class="title">{g.english_title}</span>
							<span class="meta faint">
								{formatCount(g.num_pages ?? 0)} pages · {formatCount(g.num_favorites ?? 0)}
							</span>
						</div>
					</a>
					<button class="remove" onclick={() => onRemove(g.id)} aria-label="Remove from favorites">
						<Icon name="close" size={13} />
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.favorites {
		padding: 8px 0 24px;
	}

	.head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 14px;
	}

	h2 {
		margin: 0;
		font-size: 18px;
		font-weight: 650;
		letter-spacing: -0.01em;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
		gap: 12px;
	}

	.cell {
		position: relative;
	}

	.card {
		display: flex;
		gap: 12px;
		align-items: center;
		padding: 8px;
		border-radius: var(--radius);
		border: 1px solid transparent;
		transition: background 0.12s ease, border-color 0.12s ease;
	}

	.card:hover {
		background: var(--surface);
		border-color: var(--border);
	}

	.card :global(.cover) {
		width: 54px;
		height: 76px;
		flex-shrink: 0;
	}

	.info {
		display: flex;
		flex-direction: column;
		gap: 4px;
		min-width: 0;
	}

	.title {
		font-size: 13.5px;
		font-weight: 560;
		line-height: 1.35;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.meta {
		font-size: 12px;
	}

	.remove {
		position: absolute;
		top: 12px;
		right: 12px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		border-radius: 999px;
		background: rgba(0, 0, 0, 0.55);
		color: #fff;
		opacity: 0;
		transition: opacity 0.14s ease, background 0.12s ease;
	}

	.cell:hover .remove {
		opacity: 1;
	}

	.remove:hover {
		background: var(--danger);
	}
</style>