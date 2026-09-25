<script lang="ts">
	import type { HistoryEntry } from '$lib/types';
	import { relativeDate } from '$lib/format';
	import { clearHistory, isFavorite, toggleFavorite } from '$lib/stores/library.svelte';
	import { thumbPath } from '$lib/image';
	import Icon from './Icon.svelte';

	let { entries }: { entries: HistoryEntry[] } = $props();

	function onClear() {
		clearHistory();
	}
</script>

<div class="history">
	<div class="head">
		<h2>History</h2>
		{#if entries.length > 0}
			<button class="btn btn-ghost faint" onclick={onClear}>
				<Icon name="close" size={14} />
				Clear all
			</button>
		{/if}
	</div>

	{#if entries.length === 0}
		<p class="faint">Galleries you open will appear here.</p>
	{:else}
		<ul class="list">
			{#each entries as h (h.galleryId)}
				<li>
					<a class="row" href={`/gallery/${h.galleryId}`}>
						<span class="thumb">
							<img src={thumbPath(h.thumbnail)} alt="" loading="lazy" />
						</span>
						<span class="info">
							<span class="title">{h.englishTitle}</span>
							<span class="meta faint">
								{h.numPages ? `${h.numPages} pages` : ''}
								<span aria-hidden="true">·</span>
								{relativeDate(Math.floor(h.visitedAt / 1000))}
							</span>
						</span>
					</a>
					<button
						class="icon-btn"
						class:on={isFavorite(h.galleryId)}
						onclick={() => {
							toggleFavorite({
								id: h.galleryId,
								media_id: h.mediaId,
								english_title: h.englishTitle,
								thumbnail: h.thumbnail,
								num_pages: h.numPages,
								tag_ids: [],
							});
						}}
						aria-label="Toggle favorite"
					>
						<Icon name="heart" size={15} />
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.history {
		padding: 8px 0 24px;
	}

	.head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	h2 {
		margin: 0;
		font-size: 18px;
		font-weight: 650;
		letter-spacing: -0.01em;
	}

	.list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	li {
		display: flex;
		align-items: center;
		gap: 10px;
		border-radius: var(--radius-sm);
		padding: 6px;
		transition: background 0.12s ease;
	}

	li:hover {
		background: var(--surface);
	}

	.row {
		display: flex;
		align-items: center;
		gap: 12px;
		flex: 1;
		min-width: 0;
	}

	.thumb {
		width: 44px;
		height: 62px;
		border-radius: var(--radius-sm);
		overflow: hidden;
		flex-shrink: 0;
	}

	.thumb img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		display: block;
	}

	.info {
		display: flex;
		flex-direction: column;
		gap: 3px;
		min-width: 0;
	}

	.title {
		font-weight: 560;
		font-size: 14px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.meta {
		display: flex;
		gap: 6px;
		font-size: 12px;
	}

	.icon-btn.on {
		color: var(--danger);
	}
</style>