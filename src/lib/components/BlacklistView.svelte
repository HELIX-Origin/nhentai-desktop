<script lang="ts">
	import { getSettings, updateSettings } from '$lib/stores/settings.svelte';
	import {
		getBlacklist,
		addTagByName,
		removeEntry,
		clearBlacklist,
	} from '$lib/stores/blacklist.svelte';
	import {
		getAccountState,
		addAccountBlacklist,
		removeAccountBlacklist,
	} from '$lib/stores/account.svelte';
	import Icon from './Icon.svelte';
	import EmptyState from './EmptyState.svelte';

	const s = getSettings();
	const entries = $derived(getBlacklist());
	const account = getAccountState();

	let newName = $state('');
	let newType = $state('tag');
	let syncing = $state(false);
	let syncMessage = $state<string | null>(null);

	function onAdd(event: Event) {
		event.preventDefault();
		const name = newName.trim();
		if (!name) return;
		addTagByName(name, newType);
		newName = '';
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
		<form class="add-form" onsubmit={onAdd}>
			<input class="input" placeholder="Tag name…" bind:value={newName} aria-label="Tag name to block" />
			<select class="select type" bind:value={newType} aria-label="Tag type">
				<option value="tag">Tag</option>
				<option value="artist">Artist</option>
				<option value="character">Character</option>
				<option value="parody">Parody</option>
				<option value="group">Group</option>
				<option value="language">Language</option>
				<option value="category">Category</option>
			</select>
			<button class="btn btn-primary" type="submit" disabled={!newName.trim()}>
				<Icon name="plus" size={14} />
				Block
			</button>
		</form>

		{#if entries.length === 0}
			<EmptyState
				icon="shield"
				title="Nothing blocked yet"
				description="Add tags you never want to see — they'll be excluded from results and hidden or blurred in grids."
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

	.add-form {
		display: flex;
		gap: 8px;
	}

	.add-form .input {
		flex: 1;
	}

	.add-form .select.type {
		width: auto;
		min-width: 110px;
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