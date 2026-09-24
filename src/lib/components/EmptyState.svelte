<script lang="ts">
	import Icon from './Icon.svelte';

	let { icon = 'image', title, description }: { icon?: string; title: string; description?: string } =
		$props();
</script>

<div class="empty">
	<span class="icon"><Icon name={icon} size={20} /></span>
	<h3>{title}</h3>
	{#if description}
		<p class="faint">{description}</p>
	{/if}
</div>

<style>
	.empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 72px 24px;
		text-align: center;
	}

	.icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		border-radius: 12px;
		background: var(--surface);
		color: var(--text-faint);
		margin-bottom: 6px;
	}

	h3 {
		margin: 0;
		font-size: 15px;
		font-weight: 600;
	}

	p {
		margin: 0;
		max-width: 360px;
		font-size: 13px;
	}
</style>