<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import { getSettings, updateSettings } from '$lib/stores/settings.svelte';
	import { addToHistory } from '$lib/stores/library.svelte';
	import type { GalleryDetail } from '$lib/types';
	import ReaderImage from '$lib/components/ReaderImage.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import ErrorNotice from '$lib/components/ErrorNotice.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import { thumbPath } from '$lib/image';

	const id = $derived(Number(page.params.id));
	const settings = getSettings();

	const FIT_ORDER: ('width' | 'height' | 'contain')[] = ['width', 'contain', 'height'];
	const FIT_LABEL: Record<string, string> = { width: 'Fit width', contain: 'Fit page', height: 'Fit height' };

	let gallery = $state<GalleryDetail | null>(null);
	let error = $state<string | null>(null);
	let loading = $state(true);
	let currentIdx = $state(0);
	let stageEl = $state<HTMLDivElement>();

	const pages = $derived(gallery?.pages ?? []);
	const orderedPages = $derived(settings.readerRtl ? [...pages].reverse() : pages);
	const current = $derived(orderedPages[currentIdx] ?? null);
	const shown = $derived(orderedPages.slice(Math.max(0, currentIdx - 1), currentIdx + 2));
	const isLast = $derived(currentIdx >= orderedPages.length - 1);

	function forward() {
		if (currentIdx >= orderedPages.length - 1) return;
		currentIdx++;
	}

	function back() {
		if (currentIdx <= 0) return;
		currentIdx--;
	}

	function cycleFit() {
		const next = FIT_ORDER[(FIT_ORDER.indexOf(settings.readerFit) + 1) % FIT_ORDER.length];
		updateSettings({ readerFit: next });
	}

	function onKey(e: KeyboardEvent) {
		if (e.key === 'Escape' || e.key === 'Backspace') {
			goto(`/gallery/${id}`);
		} else if (e.key === 'ArrowRight') {
			settings.readerRtl ? back() : forward();
		} else if (e.key === 'ArrowLeft') {
			settings.readerRtl ? forward() : back();
		} else if (e.key === 'f' || e.key === 'F') {
			cycleFit();
		} else if (e.key === 'r' || e.key === 'R') {
			updateSettings({ readerRtl: !settings.readerRtl });
			currentIdx = 0;
		}
	}

	onMount(() => {
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	});

	function bounce() {
		goto(`/gallery/${id}`);
	}

	function onStageKey(e: KeyboardEvent) {
		if (e.key === 'ArrowRight') settings.readerRtl ? back() : forward();
		else if (e.key === 'ArrowLeft') settings.readerRtl ? forward() : back();
	}

	$effect(() => {
		void id;
		let cancelled = false;
		loading = true;
		error = null;
		api.gallery(id)
			.then((res) => {
				if (cancelled) return;
				gallery = res;
				loading = false;
				addToHistory({
					galleryId: res.id,
					mediaId: res.media_id,
					englishTitle: res.title.english,
					thumbnail: thumbPath(res.thumbnail.path),
					numPages: res.num_pages,
					visitedAt: Date.now(),
				});
			})
			.catch((e) => {
				if (!cancelled) {
					error = String(e);
					loading = false;
				}
			});
		return () => {
			cancelled = true;
		};
	});

	$effect(() => {
		void currentIdx;
		void settings.readerFit;
		if (stageEl) {
			stageEl.scrollTop = 0;
			stageEl.scrollLeft = 0;
		}
	});
</script>

<svelte:head>
	<title>{gallery?.title.english ?? 'Reader'} — NH Desktop</title>
</svelte:head>

{#if loading && !gallery}
	<div class="page"><Loader label="Loading reader…" /></div>
{:else if error && !gallery}
	<div class="page"><ErrorNotice message={error} onretry={() => (loading = true)} /></div>
{:else if gallery}
	<div class="reader" role="group">
		<header class="bar">
			<button class="btn btn-ghost icon-now" onclick={bounce} aria-label="Back to gallery">
				<Icon name="arrow-left" size={16} />
			</button>

			<div class="info">
				<span class="title truncate">{gallery.title.english}</span>
			</div>

			<div class="controls">
				<span class="pos">{orderedPages.length ? currentIdx + 1 : 0} / {orderedPages.length}</span>
				<button class="btn btn-ghost" onclick={() => updateSettings({ readerRtl: !settings.readerRtl })} title="Toggle reading order">
					<Icon name="book" size={14} />
					{settings.readerRtl ? 'RTL' : 'LTR'}
				</button>
				<button class="btn btn-ghost" onclick={cycleFit} title="Cycle fit mode">
					<Icon name="eye" size={14} />
					{FIT_LABEL[settings.readerFit]}
				</button>
				<a class="icon-now btn btn-ghost" href={`https://nhentai.net/g/${id}`}>
					<Icon name="external" size={14} />
				</a>
			</div>
		</header>

		<div
			class="stage"
			data-fit={settings.readerFit}
			bind:this={stageEl}
			role="button"
			tabindex="0"
			onkeydown={onStageKey}
			onclick={(e) => {
				const x = e.clientX;
				const w = window.innerWidth;
				if (x < w * 0.28) settings.readerRtl ? forward() : back();
				else if (x > w * 0.72) settings.readerRtl ? back() : forward();
			}}
		>
			{#if pages.length === 0}
				<p class="faint">No page data available for this gallery.</p>
			{:else}
				{#each shown as p, i}
					<div class="layer" class:visible={p.number === current?.number}>
						<ReaderImage page={p} fit={settings.readerFit} visible={p.number === current?.number} />
					</div>
				{/each}
			{/if}
		</div>

		<div class="edge-nav">
			<button
				class="edge-btn"
				disabled={settings.readerRtl ? isLast : currentIdx <= 0}
				onclick={() => (settings.readerRtl ? forward() : back())}
				aria-label="Previous page"
			>
				<Icon name="chevron-left" size={22} />
			</button>
			<button
				class="edge-btn"
				disabled={!settings.readerRtl ? isLast : currentIdx <= 0}
				onclick={() => (!settings.readerRtl ? forward() : back())}
				aria-label="Next page"
			>
				<Icon name="chevron-right" size={22} />
			</button>
		</div>
	</div>
{/if}

<style>
	.page {
		padding: 24px;
	}

	.reader {
		position: fixed;
		inset: 0;
		background: #060609;
		display: flex;
		flex-direction: column;
		z-index: 200;
	}

	.bar {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 60px;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 0 16px;
		background: linear-gradient(to bottom, rgba(6, 6, 9, 0.92), transparent);
		z-index: 10;
		pointer-events: auto;
	}

	.icon-now {
		width: 34px;
		height: 34px;
		padding: 0;
		justify-content: center;
	}

	.info {
		flex: 1;
		min-width: 0;
		display: flex;
		align-items: center;
	}

	.title {
		font-size: 13.5px;
		font-weight: 560;
		max-width: 480px;
	}

	.controls {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.pos {
		font-size: 13px;
		color: var(--text-secondary);
		font-variant-numeric: tabular-nums;
		margin-right: 6px;
	}

	.stage {
		flex: 1;
		min-height: 0;
		min-width: 0;
		position: relative;
		overflow: auto;
	}

	.layer {
		position: absolute;
		top: 0;
		left: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stage[data-fit='width'] .layer {
		width: 100%;
		min-height: 100%;
		height: max-content;
	}

	.stage[data-fit='height'] .layer {
		height: 100%;
		min-width: 100%;
		width: max-content;
	}

	.stage[data-fit='contain'] .layer {
		inset: 0;
	}

	.layer:not(.visible) {
		visibility: hidden;
	}

	.stage p {
		position: absolute;
		margin: 0;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 13px;
		color: var(--text-faint);
	}

	.edge-nav {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		pointer-events: none;
		z-index: 5;
	}

	.edge-btn {
		pointer-events: auto;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 54px;
		height: 72px;
		margin: 0 18px;
		border-radius: var(--radius);
		background: rgba(255, 255, 255, 0.08);
		color: var(--text-secondary);
		opacity: 0.9;
		transition: background 0.12s ease, color 0.12s ease, opacity 0.12s ease;
	}

	.edge-btn:disabled {
		cursor: default;
		opacity: 0 !important;
	}

	.edge-btn:not(:disabled):hover {
		background: rgba(255, 255, 255, 0.14);
		color: var(--text);
	}

	.stage p {
		margin: 0;
		font-size: 13px;
	}
</style>