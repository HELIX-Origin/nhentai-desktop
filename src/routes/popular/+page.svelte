<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { api, GALLERIES_PER_PAGE } from '$lib/api';
	import { getSettings } from '$lib/stores/settings.svelte';
	import { getBlacklist, matchesBlacklist } from '$lib/stores/blacklist.svelte';
	import type { GalleryListItem } from '$lib/types';
	import GalleryGrid from '$lib/components/GalleryGrid.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import ErrorNotice from '$lib/components/ErrorNotice.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';
	import Pager from '$lib/components/Pager.svelte';

	const current = $derived(Number(page.url.searchParams.get('page')) || 1);
	const settings = getSettings();

	let all = $state<GalleryListItem[]>([]);
	let error = $state<string | null>(null);
	let loading = $state(true);
	let tick = $state(0);

	const hideBlocked = $derived(settings.blacklistEnabled && settings.blacklistMode === 'hide');
	const filtered = $derived(hideBlocked ? all.filter((g) => !matchesBlacklist(g)) : all);
	const numPages = $derived(Math.max(1, Math.ceil(filtered.length / GALLERIES_PER_PAGE)));
	const items = $derived(
		filtered.slice((current - 1) * GALLERIES_PER_PAGE, current * GALLERIES_PER_PAGE),
	);

	$effect(() => {
		void tick;
		let cancelled = false;
		loading = true;
		error = null;
		api.popular()
			.then((res) => {
				if (!cancelled) {
					all = res;
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

	function gotoPage(n: number) {
		goto(n <= 1 ? '/popular' : `/popular/?page=${n}`, { noScroll: true });
	}
</script>

<div class="page">
	<div class="page-head">
		<h1>Popular</h1>
		<span class="faint">Today's most-loved galleries</span>
	</div>

	{#if loading && all.length === 0}
		<Loader label="Fetching today's popular galleries…" />
	{:else if error && all.length === 0}
		<ErrorNotice message={error} onretry={() => tick++} />
	{:else if filtered.length === 0}
		<EmptyState icon="flame" title="Nothing trending right now" description="Check back soon." />
	{:else}
		<GalleryGrid galleries={items} />
		<Pager page={current} numPages={numPages} ongoto={gotoPage} />
	{/if}
</div>