<script lang="ts">
	import { api } from '$lib/api';
	import type { GalleryListItem } from '$lib/types';
	import GalleryGrid from '$lib/components/GalleryGrid.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import ErrorNotice from '$lib/components/ErrorNotice.svelte';
	import EmptyState from '$lib/components/EmptyState.svelte';

	let items = $state<GalleryListItem[]>([]);
	let error = $state<string | null>(null);
	let loading = $state(true);
	let tick = $state(0);

	$effect(() => {
		void tick;
		let cancelled = false;
		loading = true;
		error = null;
		api.popular()
			.then((res) => {
				if (!cancelled) {
					items = res;
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
</script>

<div class="page">
	<div class="page-head">
		<h1>Popular</h1>
		<span class="faint">Today's most-loved galleries</span>
	</div>

	{#if loading}
		<Loader label="Fetching today's popular galleries…" />
	{:else if error}
		<ErrorNotice message={error} onretry={() => tick++} />
	{:else if items.length === 0}
		<EmptyState icon="flame" title="Nothing trending right now" description="Check back soon." />
	{:else}
		<GalleryGrid galleries={items} />
	{/if}
</div>