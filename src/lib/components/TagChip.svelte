<script lang="ts">
	let { name, type = 'tag', active = false, disabled = false, onclick }: {
		name: string;
		type?: string;
		active?: boolean;
		disabled?: boolean;
		onclick?: (e: MouseEvent) => void;
	} = $props();
</script>

{#if onclick}
	<button
		class="chip"
		class:active
		class:disabled
		data-type={type}
		onclick={onclick}
		type="button"
	>
		<span class="dot" aria-hidden="true"></span>
		{name}
	</button>
{:else}
	<span class="chip" class:active data-type={type}>
		<span class="dot" aria-hidden="true"></span>
	</span>
{/if}

<style>
	.chip {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 4px 10px;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: var(--bg-elevated);
		color: var(--text-secondary);
		font-size: 12.5px;
		font-weight: 520;
		line-height: 1.4;
		transition: background 0.1s ease, border-color 0.1s ease, color 0.1s ease;
		white-space: nowrap;
	}

	button.chip {
		cursor: pointer;
	}

	button.chip:hover {
		background: var(--surface-hover);
		color: var(--text);
		border-color: var(--border-strong);
	}

	.chip.active {
		background: var(--accent-soft);
		border-color: var(--accent-border);
		color: var(--text);
	}

	.chip.disabled {
		opacity: 0.45;
		cursor: not-allowed;
	}

	.dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--text-faint);
		flex-shrink: 0;
	}

	.chip[data-type='artist'] .dot {
		background: #f6b545;
	}

	.chip[data-type='character'] .dot {
		background: #3ecf8e;
	}

	.chip[data-type='parody'] .dot {
		background: #7c5cff;
	}

	.chip[data-type='group'] .dot {
		background: #35c9d4;
	}

	.chip[data-type='language'] .dot {
		background: #9d9daa;
	}

	.chip[data-type='category'] .dot {
		background: #f4576b;
	}
</style>