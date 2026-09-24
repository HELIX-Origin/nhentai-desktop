<script lang="ts">
	import Icon from './Icon.svelte';

	let { page, numPages, ongoto }: { page: number; numPages: number; ongoto: (p: number) => void } =
		$props();

	const atStart = $derived(page <= 1);
	const atEnd = $derived(page >= numPages);
</script>

<nav class="pager" aria-label="Pagination">
	<button class="btn" disabled={atStart} onclick={() => ongoto(page - 1)}>
		<Icon name="chevron-left" size={15} />
		Prev
	</button>
	<span class="page-info">
		Page <b>{page}</b> of <b>{numPages > 0 ? numPages : 1}</b>
	</span>
	<button class="btn" disabled={atEnd} onclick={() => ongoto(page + 1)}>
		Next
		<Icon name="chevron-right" size={15} />
	</button>
</nav>

<style>
	.pager {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 18px;
		padding: 24px 0 8px;
	}

	.page-info {
		font-size: 13px;
		color: var(--text-secondary);
		min-width: 130px;
		text-align: center;
	}

	.page-info b {
		color: var(--text);
		font-variant-numeric: tabular-nums;
	}
</style>