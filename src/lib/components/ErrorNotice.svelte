<script lang="ts">
	import Icon from './Icon.svelte';

	let { message, onretry }: { message: string; onretry?: () => void } = $props();
</script>

<div class="error" role="alert">
	<div class="row">
		<Icon name="alert" size={16} />
		<div class="body">
			<span class="title">Something went wrong</span>
			<span class="faint">{message}</span>
		</div>
	</div>
	{#if onretry}
		<button class="btn" onclick={onretry}>Retry</button>
	{/if}
</div>

<style>
	.error {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
		padding: 14px 16px;
		border-radius: var(--radius);
		border: 1px solid var(--danger);
		background: var(--danger-soft);
		color: var(--danger);
	}

	.row {
		display: flex;
		align-items: center;
		gap: 12px;
		min-width: 0;
	}

	.body {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
	}

	.title {
		font-weight: 600;
	}

	.faint {
		color: var(--text-secondary);
		font-size: 13px;
	}

	.error .btn {
		flex-shrink: 0;
		background: var(--danger);
		border-color: var(--danger);
		color: #fff;
	}

	.error .btn:hover {
		background: var(--danger);
		opacity: 0.9;
	}
</style>