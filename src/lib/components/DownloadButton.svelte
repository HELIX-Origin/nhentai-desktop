<script lang="ts">
	import { enqueueDownload, getServiceJobs } from '$lib/stores/service.svelte';
	import { getAccountState } from '$lib/stores/account.svelte';
	import Icon from './Icon.svelte';

	let { galleryId }: { galleryId: number } = $props();

	const account = $derived(getAccountState());
	const jobs = $derived(getServiceJobs());

	const FORMATS = ['zip', 'cbz', 'torrent'] as const;
	type Format = (typeof FORMATS)[number];

	let open = $state(false);
	let format = $state<Format>('zip');
	let jobId = $state<number | null>(null);
	let started = $state(false);
	let done = $state(false);
	let failed = $state(false);
	let message = $state('');
	let progress = $state<number | null>(null);

	const job = $derived(
		jobId === null ? null : (jobs.find((j) => j.jobId === jobId) ?? null),
	);

	$effect(() => {
		const j = job;
		if (!j || j.kind !== 'download') return;
		if (j.state === 'running') {
			progress = j.total && j.total > 0 ? Math.round(((j.done ?? 0) / j.total) * 100) : null;
		} else if (j.state === 'finished') {
			done = true;
			started = false;
			failed = false;
			message = j.message ?? 'Download finished';
		} else if (j.state === 'failed') {
			failed = true;
			started = false;
			done = false;
			message = j.error ?? 'Download failed';
		}
	});

	async function start(fmt: Format) {
		format = fmt;
		open = false;
		started = true;
		done = false;
		failed = false;
		message = '';
		progress = null;
		try {
			jobId = await enqueueDownload(galleryId, fmt);
		} catch (e) {
			failed = true;
			started = false;
			message = String(e);
		}
	}

	function onMenuClick(e: MouseEvent) {
		const t = e.target as HTMLElement | null;
		if (t?.closest('.menu')) return;
		open = !open;
	}
</script>

<div class="download">
	{#if failed}
		<button class="btn btn-danger" class:open={open} onclick={() => (failed = false)} title={message}>
			<Icon name="alert" size={15} />
			Retry
		</button>
		<span class="note err" title={message}>{message}</span>
	{:else if done}
		<button
			class="btn"
			class:open={open}
			onclick={onMenuClick}
			title="Download another format"
			aria-label="Download"
		>
			<Icon name="check" size={15} />
			Downloaded
		</button>
	{:else if started}
		<button class="btn" disabled title="Downloading…">
			<span class="spin"><Icon name="download" size={15} /></span>
			{progress !== null ? `${progress}%` : 'Queued'}
		</button>
	{:else}
		<button class="btn" class:open={open} onclick={onMenuClick} aria-label="Download" aria-haspopup="menu">
			<Icon name="download" size={15} />
			Download
		</button>
	{/if}
	<span class="note faint">
		{#if account.keyStatus.configured}
			{format === 'zip' ? 'ZIP' : format === 'cbz' ? 'CBZ' : 'Torrent'}
		{:else}
			Requires an API key in Settings
		{/if}
	</span>

	{#if open}
		<div class="menu" role="menu">
			{#each FORMATS as fmt}
				<button class="item" role="menuitem" onclick={() => start(fmt)}>
					<span class="name">{fmt.toUpperCase()}</span>
					<span class="desc">
						{fmt === 'zip' ? 'Zip archive' : fmt === 'cbz' ? 'Comic book' : 'Seeder file'}
					</span>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.download {
		position: relative;
		display: inline-flex;
		align-items: center;
		gap: 8px;
	}

	.btn.open {
		border-color: var(--accent);
		color: var(--accent);
	}

	.menu {
		position: absolute;
		top: calc(100% + 6px);
		left: 0;
		z-index: 20;
		min-width: 180px;
		padding: 4px;
		border-radius: var(--radius);
		border: 1px solid var(--border-strong);
		background: var(--bg-elevated);
		box-shadow: var(--shadow-lg);
	}

	.item {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 1px;
		width: 100%;
		padding: 7px 10px;
		border-radius: var(--radius-sm);
		border: none;
		background: none;
		color: var(--text);
		cursor: pointer;
		text-align: left;
	}

	.item:hover {
		background: var(--surface-hover);
	}

	.name {
		font-size: 13px;
		font-weight: 600;
	}

	.desc {
		font-size: 11.5px;
		color: var(--text-faint);
	}

	.note {
		font-size: 12px;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.note.err {
		color: var(--danger);
		max-width: 260px;
	}

	.spin {
		display: inline-flex;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>