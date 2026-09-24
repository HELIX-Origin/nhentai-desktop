<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api';
	import type { GalleryList } from '$lib/types';
	import GalleryGrid from '$lib/components/GalleryGrid.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import ErrorNotice from '$lib/components/ErrorNotice.svelte';
	import Pager from '$lib/components/Pager.svelte';

	const current = $derived(Number(page.url.searchParams.get('page')) || 1);

	let data = $state<GalleryList | null>(null);
	let error = $state<string | null>(null);
	let loading = $state(true);
	let requestId = 0;
	let tick = $state(0);

	$effect(() => {
		void tick;
		const id = ++requestId;
		loading = true;
		error = null;
		api.newGalleries(current)
			.then((res) => {
				if (id === requestId) {
					data = res;
					loading = false;
				}
			})
			.catch((e) => {
				if (id === requestId) {
					error = String(e);
					loading = false;
				}
			});
	});

	function gotoPage(n: number) {
		goto(n <= 1 ? '/' : `/?page=${n}`, { noScroll: true });
	}

	function retry() {
		tick++;
	}
</script>

<div class="page">
	<div class="page-head">
		<h1>Latest</h1>
		<span class="faint">Recently uploaded galleries</span>
	</div>

	{#if loading && !data}
		<Loader label="Loading latest galleries…" />
	{:else if error && !data}
		<ErrorNotice message={error} onretry={retry} />
	{:else if data}
		<GalleryGrid galleries={data.result} />
		<Pager page={current} numPages={data.num_pages} ongoto={gotoPage} />
	{/if}
</div>