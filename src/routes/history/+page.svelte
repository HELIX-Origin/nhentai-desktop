<script lang="ts">
	import { getHistory } from '$lib/stores/library.svelte';
	import HistoryView from '$lib/components/HistoryView.svelte';
</script>

<div class="page">
	<HistoryView entries={getHistory()} />
</div>