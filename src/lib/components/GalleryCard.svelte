<script lang="ts">
	import { getSettings } from '$lib/stores/settings.svelte';
	import { getBlacklist } from '$lib/stores/blacklist.svelte';
	import { isFavorite, toggleFavorite } from '$lib/stores/library.svelte';
	import { formatCount } from '$lib/format';
	import type { GalleryListItem } from '$lib/types';
	import Icon from './Icon.svelte';
	import CoverImage from './CoverImage.svelte';

	let { gallery }: { gallery: GalleryListItem } = $props();

	const s = getSettings();
	const blocked = $derived(
		s.blacklistEnabled && getBlacklist().some((e) => gallery.tag_ids.includes(e.id)),
	);
	const fav = $derived(isFavorite(gallery.id));
	const ratio = $derived(
		gallery.thumbnail_width && gallery.thumbnail_height
			? gallery.thumbnail_width / gallery.thumbnail_height
			: 0.6667,
	);

	function onFav(event: MouseEvent) {
		event.preventDefault();
		event.stopPropagation();
		toggleFavorite(gallery);
	}
</script>

<a class="card" class:blurred={s.blacklistMode === 'blur' && blocked} href={`/gallery/${gallery.id}`}>
	<div class="thumb">
		<CoverImage src={gallery.thumbnail} alt={gallery.english_title} ratio={ratio} />
		{#if blocked}
			<span class="blocked-tag"><Icon name="shield" size={11} /> blocked</span>
		{/if}
		<button class="fav" class:on={fav} onclick={onFav} aria-label={fav ? 'Remove from favorites' : 'Add to favorites'}>
			<Icon name="heart" size={15} />
		</button>
	</div>

	<h3 class="title" title={gallery.english_title}>{gallery.english_title}</h3>
	<div class="meta">
		<span>{gallery.num_pages ?? 0} pages</span>
		<span aria-hidden="true">·</span>
		<span>{formatCount(gallery.num_favorites ?? 0)} favs</span>
	</div>
</a>

<style>
	.card {
		display: flex;
		flex-direction: column;
		gap: 8px;
		border-radius: var(--radius);
		padding: 6px;
		transition: background 0.12s ease;
		min-width: 0;
	}

	.card:hover {
		background: var(--surface);
	}

	.thumb {
		position: relative;
		border-radius: var(--radius-sm);
		overflow: hidden;
	}

	.thumb :global(.cover) {
		border-radius: var(--radius-sm);
	}

	.card.blurred .thumb :global(img) {
		filter: blur(14px);
		transform: scale(1.08);
	}

	.fav {
		position: absolute;
		top: 8px;
		right: 8px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border-radius: 999px;
		background: rgba(0, 0, 0, 0.55);
		color: #fff;
		opacity: 0;
		transform: translateY(-3px);
		transition:
			opacity 0.14s ease,
			transform 0.14s ease,
			background 0.12s ease;
		backdrop-filter: blur(4px);
	}

	.card:hover .fav,
	.fav:focus-visible {
		opacity: 1;
		transform: none;
	}

	.fav:hover {
		background: rgba(0, 0, 0, 0.75);
		color: #fff;
	}

	.fav.on {
		color: var(--danger);
		opacity: 1;
		transform: none;
	}

	.blocked-tag {
		position: absolute;
		bottom: 8px;
		left: 8px;
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 3px 7px;
		border-radius: 999px;
		background: rgba(0, 0, 0, 0.6);
		color: var(--text);
		font-size: 10.5px;
		font-weight: 600;
		backdrop-filter: blur(4px);
	}

	.title {
		margin: 0;
		font-size: 13px;
		font-weight: 560;
		line-height: 1.35;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		min-height: 0;
		padding: 0 4px;
	}

	.meta {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0 4px;
		font-size: 12px;
		color: var(--text-faint);
	}
</style>