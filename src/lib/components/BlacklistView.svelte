<script lang="ts">
	import { getSettings, updateSettings } from '$lib/stores/settings.svelte';
	import {
		getBlacklist,
		addTag,
		removeEntry,
		removeByRef,
		clearBlacklist,
	} from '$lib/stores/blacklist.svelte';
	import {
		getAccountState,
		addAccountBlacklist,
		removeAccountBlacklist,
	} from '$lib/stores/account.svelte';
	import { api } from '$lib/api';
	import Icon from './Icon.svelte';
	import EmptyState from './EmptyState.svelte';
	import type { Paginated, Tag } from '$lib/types';

	const s = getSettings();
	const entries = $derived(getBlacklist());
	const account = $derived(getAccountState());

	const TYPES = [
		{ value: 'tag', label: 'Tag' },
		{ value: 'artist', label: 'Artist' },
		{ value: 'character', label: 'Character' },
		{ value: 'parody', label: 'Parody' },
		{ value: 'group', label: 'Group' },
		{ value: 'language', label: 'Language' },
		{ value: 'category', label: 'Category' },
	];

	let pickerType = $state('tag');
	let pickerPage = $state(1);
	let options = $state<Paginated<Tag> | null>(null);
	let pickerLoading = $state(false);
	let pickerError = $state<string | null>(null);
	let syncing = $state(false);
	let syncMessage = $state<string | null>(null);

	$effect(() => {
		const type = pickerType;
		const page = pickerPage;
		let cancelled = false;
		pickerLoading = true;
		pickerError = null;
		options = null;
		api
			.tagsByType(type, 'popular', page, 24)
			.then((res) => {
				if (!cancelled) options = res;
			})
			.catch((e) => {
				if (!cancelled) pickerError = String(e);
			})
			.finally(() => {
				if (!cancelled) pickerLoading = false;
			});
		return () => {
			cancelled = true;
		};
	});

	function changeType(type: string) {
		pickerType = type;
		pickerPage = 1;
	}

	function isBlocked(tag: Tag): boolean {
		return entries.some((e) => e.name === tag.name && e.type === tag.type);
	}

	function onToggle(tag: Tag) {
		if (isBlocked(tag)) {
			removeByRef({ id: tag.id, name: tag.name, type: tag.type, slug: tag.slug });
		} else {
			addTag(tag);
		}
	}

	async function onSyncToAccount() {
		syncing = true;
		syncMessage = null;
		try {
			const ids = entries.filter((e) => e.id > 0).map((e) => e.id);
			if (ids.length > 0) await addAccountBlacklist(ids);
			syncMessage = `Synced ${ids.length} tag(s) to your nhentai account.`;
		} catch (e) {
			syncMessage = `Sync failed: ${String(e)}`;
		} finally {
			syncing = false;
		}
	}

	async function onRemove(index: number) {
		const entry = entries[index];
		removeEntry(index);
		if (account.keyStatus.configured && entry.id > 0) {
			try {
				await removeAccountBlacklist([entry.id]);
			} catch {
				// local removal already handled
			}
		}
	}
</script>

<div class="blacklist">
	<div class="head">
		<h2>Blacklist</h2>
		<span class="count faint">{entries.length} blocked</span>
	</div>

	<section class="panel">
		<div class="panel-row">
			<div>
				<div class="row-title">Master switch</div>
				<p class="faint">Applies blacklisted tags everywhere — searches and browse views.</p>
			</div>
			<button
				class="switch"
				class:on={s.blacklistEnabled}
				onclick={() => updateSettings({ blacklistEnabled: !s.blacklistEnabled })}
				role="switch"
				aria-checked={s.blacklistEnabled}
				aria-label="Toggle blacklist"
			>
				<span class="knob"></span>
			</button>
		</div>

		<div class="panel-row">
			<div>
				<div class="row-title">Treatment</div>
				<p class="faint">Hide blocked galleries entirely, or keep the grid intact and blur them.</p>
			</div>
			<select class="select" value={s.blacklistMode} onchange={(e) => updateSettings({ blacklistMode: e.currentTarget.value as 'hide' | 'blur' })}>
				<option value="hide">Hide</option>
				<option value="blur">Blur</option>
			</select>
		</div>
	</section>

	<section class="panel">
		<div class="row-title">Pick tags to block</div>
		<p class="faint">Choose from popular {pickerType}s — tap + to add, − to remove.</p>

		<div class="type-tabs" role="tablist" aria-label="Tag type">
			{#each TYPES as t (t.value)}
				<button
					class="type-tab"
					class:on={pickerType === t.value}
					role="tab"
					aria-selected={pickerType === t.value}
					onclick={() => changeType(t.value)}
				>
					{t.label}
				</button>
			{/each}
		</div>

		{#if pickerError}
			<p class="picker-error">{pickerError}</p>
		{:else if pickerLoading}
			<p class="faint picker-loading">Loading options…</p>
		{:else if options}
			<ul class="options">
				{#each options.result as tag (tag.id)}
					<button
						class="option-pill"
						class:blocked={isBlocked(tag)}
						onclick={() => onToggle(tag)}
						aria-pressed={isBlocked(tag)}
					>
						<span class="dot" data-type={pickerType}></span>
						<span class="opt-name">{tag.name}</span>
						<span class="opt-count">{tag.count.toLocaleString()}</span>
						<span class="opt-action">
							{#if isBlocked(tag)}
								<Icon name="minus" size={13} />
							{:else}
								<Icon name="plus" size={13} />
							{/if}
						</span>
					</button>
				{/each}
			</ul>

			<div class="pager">
				<button class="btn" onclick={() => (pickerPage -= 1)} disabled={pickerPage <= 1}>
					<Icon name="chevron-left" size={14} />
					Prev
				</button>
				<span class="faint">Page {pickerPage} of {options.num_pages}</span>
				<button class="btn" onclick={() => (pickerPage += 1)} disabled={pickerPage >= options.num_pages}>
					Next
					<Icon name="chevron-right" size={14} />
				</button>
			</div>
		{/if}
	</section>

	<section class="panel">
		<div class="row-title">Blocked</div>
		{#if entries.length === 0}
			<EmptyState
				icon="shield"
				title="Nothing blocked yet"
				description="Pick tags above — they'll be excluded from results and hidden or blurred in grids."
			/>
		{:else}
			<ul class="list">
				{#each entries as e, i (i)}
					<li>
						<span class="entry">
							<span class="type-pill">{e.type}</span>
							<span class="name">{e.name}</span>
							{#if e.count}
								<span class="count-chip faint">{e.count}</span>
							{/if}
						</span>
						<button class="icon-btn" onclick={() => onRemove(i)} aria-label={`Unblock ${e.name}`}>
							<Icon name="close" size={14} />
						</button>
					</li>
				{/each}
			</ul>
		{/if}
	</section>

	{#if account.keyStatus.configured}
		<section class="panel account">
			<div class="panel-row">
				<div>
					<div class="row-title">Sync with nhentai account</div>
					<p class="faint">
						Push your local blacklist to <b>{account.user?.username ?? 'your account'}</b> so
						searches on the site respect it too.
					</p>
					{#if syncMessage}
						<p class="sync" class:error={syncMessage.startsWith('Sync failed')}>{syncMessage}</p>
					{/if}
				</div>
			</div>
			<button class="btn" onclick={onSyncToAccount} disabled={syncing || entries.length === 0}>
				<Icon name="user" size={14} />
				{syncing ? 'Syncing…' : `Sync ${entries.filter((e) => e.id > 0).length} to account`}
			</button>
		</section>
	{/if}

	{#if entries.length > 0}
		<div class="foot">
			<button class="btn btn-danger" onclick={clearBlacklist}>
				<Icon name="close" size={14} />
				Clear all blocked tags
			</button>
		</div>
	{/if}
</div>

<style>
	.blacklist {
		padding: 8px 0 24px;
		max-width: 760px;
	}

	.head {
		display: flex;
		align-items: baseline;
		gap: 10px;
		margin-bottom: 14px;
	}

	h2 {
		margin: 0;
		font-size: 18px;
		font-weight: 650;
		letter-spacing: -0.01em;
	}

	.count {
		font-size: 13px;
	}

	.panel {
		border: 1px solid var(--border);
		border-radius: var(--radius);
		background: var(--bg-elevated);
		padding: 16px;
		margin-bottom: 14px;
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.panel-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
	}

	.panel-row > div {
		min-width: 0;
	}

	.row-title {
		font-weight: 600;
		font-size: 14px;
		margin-bottom: 2px;
	}

	.panel p {
		margin: 0;
		font-size: 12.5px;
	}

	.switch {
		position: relative;
		width: 42px;
		height: 24px;
		border-radius: 999px;
		background: var(--surface-active);
		border: 1px solid var(--border-strong);
		transition: background 0.15s ease, border-color 0.15s ease;
		flex-shrink: 0;
	}

	.switch.on {
		background: var(--accent);
		border-color: var(--accent);
	}

	.knob {
		position: absolute;
		top: 3px;
		left: 3px;
		width: 16px;
		height: 16px;
		border-radius: 50%;
		background: #fff;
		transition: transform 0.15s ease;
	}

	.switch.on .knob {
		transform: translateX(18px);
	}

	.type-tabs {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.type-tab {
		padding: 5px 12px;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: var(--surface);
		color: var(--text);
		font-size: 12.5px;
		font-weight: 550;
		transition: background 0.12s ease, border-color 0.12s ease, color 0.12s ease;
	}

	.type-tab:hover {
		background: var(--surface-hover);
	}

	.type-tab.on {
		background: var(--accent-soft);
		border-color: var(--accent);
		color: var(--accent-hover);
	}

	.options {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.option-pill {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		padding: 5px 9px;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: var(--surface);
		color: var(--text);
		font-size: 13px;
		transition: background 0.12s ease, border-color 0.12s ease;
		max-width: 100%;
	}

	.option-pill:hover {
		background: var(--surface-hover);
		border-color: var(--border-strong);
	}

	.option-pill.blocked {
		background: var(--accent-soft);
		border-color: var(--accent);
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.dot[data-type='artist'] {
		background: #f6b545;
	}
	.dot[data-type='character'] {
		background: #3ecf8e;
	}
	.dot[data-type='parody'] {
		background: #7c5cff;
	}
	.dot[data-type='group'] {
		background: #35c9d4;
	}
	.dot[data-type='language'] {
		background: #9d9daa;
	}
	.dot[data-type='category'] {
		background: #f4576b;
	}
	.dot[data-type='tag'] {
		background: var(--text-faint);
	}

	.opt-name {
		font-weight: 550;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.opt-count {
		font-size: 11.5px;
		color: var(--text-faint);
		font-variant-numeric: tabular-nums;
	}

	.opt-action {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: var(--surface-active);
		color: var(--text-soft);
		flex-shrink: 0;
	}

	.option-pill.blocked .opt-action {
		background: var(--accent);
		color: #fff;
	}

	.picker-error {
		color: var(--danger);
		font-size: 12.5px;
	}

	.pager {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 10px;
	}

	.list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	li {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 10px;
		padding: 8px 10px;
		border-radius: var(--radius-sm);
		transition: background 0.12s ease;
	}

	li:hover {
		background: var(--surface-hover);
	}

	.entry {
		display: flex;
		align-items: center;
		gap: 8px;
		min-width: 0;
	}

	.type-pill {
		font-size: 10px;
		font-weight: 650;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		color: var(--accent-hover);
		background: var(--accent-soft);
		border-radius: 4px;
		padding: 2px 6px;
		flex-shrink: 0;
	}

	.name {
		font-weight: 550;
		font-size: 13.5px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.count-chip {
		font-size: 12px;
		font-variant-numeric: tabular-nums;
	}

	.sync {
		margin-top: 8px !important;
		color: var(--success);
		font-size: 12.5px;
	}

	.sync.error {
		color: var(--danger);
	}

	.account .btn {
		align-self: flex-start;
	}

	.foot {
		display: flex;
		justify-content: flex-end;
	}
</style>