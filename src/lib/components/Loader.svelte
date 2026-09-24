<script lang="ts">
	let { label = 'Loading…' }: { label?: string } = $props();
</script>

<div class="loader" role="status">
	<div class="spinner"></div>
	<span class="faint">{label}</span>
</div>

<style>
	.loader {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 14px;
		padding: 64px 0;
	}

	.spinner {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		border: 2px solid var(--border-strong);
		border-top-color: var(--accent);
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>