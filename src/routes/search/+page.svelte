<script lang="ts">
	import { page } from '$app/state';
	import { api } from '$lib/api';
	import { buildQuery, summarizeFilters } from '$lib/query';
	import { buildServerExcludes } from '$lib/stores/blacklist.svelte';
	import type { FilterModel, GalleryList } from '$lib/types';
	import GalleryGrid from '$lib/components/GalleryGrid.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import ErrorNotice from '$lib/components/ErrorNotice.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import Pager from '$lib/components/Pager.svelte';
	import FilterPanel from '$lib/components/FilterPanel.svelte';
	import Drawer from '$lib/components/Drawer.svelte';
	import Icon from '$lib/components/Icon.svelte';

	function freshModel(): FilterModel {
		return {
			query: '',
			included: [],
			excluded: [],
			language: undefined,
			category: undefined,
			minPages: undefined,
			maxPages: undefined,
			sort: 'date',
		};
	}

	const urlQuery = $derived(page.url.searchParams.get('q') ?? '');
	let lastIncoming = $state('');

	let model = $state<FilterModel>(freshModel());
	let appliedQuery = $state('');
	let pageNum = $state(1);
	let results = $state<GalleryList | null>(null);
	let error = $state<string | null>(null);
	let loading = $state(false);
	let tick = $state(0);
	let drawerOpen = $state(false);

	const filterCount = $derived(summarizeFilters(model).count);

	$effect(() => {
		if (urlQuery && urlQuery !== lastIncoming) {
			lastIncoming = urlQuery;
			model.query = urlQuery;
			run();
		}
	});

	function run() {
		const base = buildQuery(model);
		const excludes = buildServerExcludes();
		appliedQuery = [base, excludes].filter(Boolean).join(' ');
		pageNum = 1;
		tick++;
	}

	$effect(() => {
		if (!appliedQuery) return;
		let cancelled = false;
		loading = true;
		error = null;
		api.search(appliedQuery, model.sort, pageNum)
			.then((res) => {
				if (!cancelled) {
					results = res;
					loading = false;
				}
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

	function submit(event: Event) {
		event.preventDefault();
		run();
	}

	function updateModel(m: FilterModel) {
		model = m;
	}

	function resetAll() {
		model = freshModel();
		lastIncoming = '';
		run();
	}
</script>

<div class="page">
	<div class="page-head">
		<h1>Search</h1>
		{#if appliedQuery}
			<span class="query-pill truncate secondary" title={appliedQuery}>{appliedQuery}</span>
		{/if}
		<span class="spacer"></span>
		<button class="btn" class:active={filterCount > 0} onclick={() => (drawerOpen = true)}>
			<Icon name="filter" size={15} />
			Filters
			{#if filterCount > 0}
				<span class="count">{filterCount}</span>
			{/if}
		</button>
	</div>

	<form class="search-bar" onsubmit={submit} role="search">
		<Icon name="search" size={17} />
		<input
			class="search-input"
			placeholder="Search titles, tags, artists, parodies…  e.g. blue archive, -tag:loli, language:english"
			bind:value={model.query}
			aria-label="Search query"
		/>
		<button class="btn btn-primary" type="submit" disabled={!model.query.trim() && filterCount === 0}>
			Search
		</button>
	</form>

	{#if loading && !results}
		<Loader label="Searching…" />
	{:else if error && !results}
		<ErrorNotice message={error} onretry={() => tick++} />
	{:else if !appliedQuery}
		<EmptyState
			icon="search"
			title="Refine instead of scrolling"
			description="Use tags, language, category and page-count filters to cut the haystack down to exactly what you want."
		/>
	{:else if results && results.result.length === 0}
		<EmptyState icon="search" title="No results" description="Try fewer or broader filters." />
	{:else if results}
		<GalleryGrid galleries={results.result} />
		<Pager page={pageNum} numPages={results.num_pages} ongoto={(n) => ((pageNum = n), (tick++))} />
	{/if}
</div>

<Drawer title="Search filters" open={drawerOpen} width={400} onclose={() => (drawerOpen = false)}>
	<FilterPanel model={model} onupdate={updateModel} />
</Drawer>

<style>
	.query-pill {
		max-width: 340px;
		font-size: 12.5px;
		color: var(--text-secondary);
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 999px;
		padding: 3px 10px;
	}

	.btn.active {
		border-color: var(--accent-border);
		color: var(--text);
		background: var(--accent-soft);
	}

	.count {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 18px;
		height: 18px;
		padding: 0 5px;
		border-radius: 9px;
		background: var(--accent);
		color: #fff;
		font-size: 11px;
		font-weight: 650;
	}

	.search-bar {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 12px;
		border-radius: var(--radius);
		border: 1px solid var(--border-strong);
		background: var(--bg-elevated);
		margin-bottom: 20px;
		color: var(--text-faint);
		transition: border-color 0.12s ease, box-shadow 0.12s ease;
	}

	.search-bar:focus-within {
		border-color: var(--accent);
		box-shadow: 0 0 0 3px var(--accent-soft);
	}

	.search-input {
		flex: 1;
		border: none;
		background: none;
		outline: none;
		color: var(--text);
		font-size: 14px;
		min-width: 0;
	}

	.search-input::placeholder {
		color: var(--text-faint);
	}
</style>