<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '$lib/components/Icon.svelte';
	import { loadSettings, getSettings } from '$lib/stores/settings.svelte';
	import { loadLibrary } from '$lib/stores/library.svelte';
	import { loadBlacklist, getBlacklist } from '$lib/stores/blacklist.svelte';
	import { initAccount, getAccountState } from '$lib/stores/account.svelte';
	import { cacheInit } from '$lib/cache';
	import '../lib/design/base.css';

	let { children } = $props();

	const isChildWindow = $derived(
		page.url.pathname.startsWith('/installer')
	);

	const nav = [
		{ href: '/', label: 'Latest', icon: 'grid' },
		{ href: '/popular', label: 'Popular', icon: 'flame' },
		{ href: '/favorites', label: 'Favorites', icon: 'heart' },
		{ href: '/history', label: 'History', icon: 'clock' },
		{ href: '/blacklist', label: 'Blacklist', icon: 'shield' },
		{ href: '/settings', label: 'Settings', icon: 'settings' },
	];

	const settings = getSettings();
	const blacklist = $derived(getBlacklist());
	const account = getAccountState();

	let quickQuery = $state('');
	let ready = $state(false);

	function onQuickSearch(event: Event) {
		event.preventDefault();
		const q = quickQuery.trim();
		if (!q) return;
		goto(`/search?${new URLSearchParams({ q })}`);
	}

	function onModifyInstallation() {
		invoke('open_maintenance_window');
	}

	onMount(() => {
		if (isChildWindow) {
			ready = true;
			return;
		}
		Promise.all([
			cacheInit(),
			loadSettings(),
			loadLibrary(),
			loadBlacklist(),
			initAccount(),
		]).finally(() => {
			ready = true;
		});
	});
</script>

<svelte:head>
	<title>nhentai</title>
	<meta name="color-scheme" content="dark" />
</svelte:head>

{#if isChildWindow}
	{@render children()}
{:else}
<div class="shell">
	<aside class="sidebar">
		<div class="brand">
			<span class="brand-mark">n</span>
			<span class="brand-name">nhentai</span>
		</div>

		<nav class="nav" aria-label="Primary">
			{#each nav as item}
				<a
					class="nav-item"
					class:active={page.url.pathname === item.href}
					href={item.href}
				>
					<Icon name={item.icon} size={18} />
					<span class="nav-label">{item.label}</span>
				</a>
			{/each}
		</nav>

		<div class="sidebar-foot">
			<button class="modify-btn" onclick={onModifyInstallation} title="Repair or uninstall nhentai">
				<Icon name="settings" size={14} />
				<span class="nav-label">Modify installation</span>
			</button>
			<span class="faint">v0.1.0</span>
		</div>
	</aside>

	<div class="main">
		<header class="topbar">
			<form class="quick-search" onsubmit={onQuickSearch} role="search">
				<Icon name="search" size={16} />
				<input
					class="quick-input"
					placeholder="Search galleries…  (or open Search)"
					bind:value={quickQuery}
					aria-label="Quick search"
				/>
				<kbd>Enter</kbd>
			</form>

			<div class="topbar-actions">
				<a class="chip-btn" href="/blacklist" title="Blacklisted tags" aria-label="Blacklisted tags">
					<Icon name="shield" size={16} />
					{#if blacklist.length > 0}
						<span class="badge" class:off={!settings.blacklistEnabled}>
							{blacklist.length}
						</span>
					{/if}
				</a>

				<a class="account-chip" href="/settings" title="Account & settings">
					{#if account.user}
						<span class="avatar">{account.user.username[0]?.toUpperCase()}</span>
						<span class="account-name">{account.user.username}</span>
					{:else}
						<Icon name="user" size={16} />
						<span class="account-name">Sign in</span>
					{/if}
				</a>
			</div>
		</header>

		<main class="content">
			{#if ready}
				{@render children()}
			{:else}
				<div class="boot-splash">
					<div class="boot-spinner"></div>
				</div>
			{/if}
		</main>
	</div>
</div>
{/if}

<style>
	.shell {
		display: flex;
		height: 100vh;
	}

	.sidebar {
		width: var(--sidebar-w);
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
		border-right: 1px solid var(--border);
		background: var(--bg-elevated);
		padding: 0 12px 12px;
	}

	.brand {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 16px 8px 18px;
	}

	.brand-mark {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 30px;
		height: 30px;
		border-radius: 8px;
		background: var(--accent);
		color: #fff;
		font-weight: 700;
		font-size: 14px;
		letter-spacing: 0.02em;
	}

	.brand-name {
		font-size: 15px;
		font-weight: 650;
		letter-spacing: -0.01em;
	}

	.nav {
		display: flex;
		flex-direction: column;
		gap: 2px;
		margin-top: 4px;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 8px 10px;
		border-radius: var(--radius-sm);
		color: var(--text-secondary);
		font-weight: 520;
		font-size: 13.5px;
		transition:
			background 0.12s ease,
			color 0.12s ease;
	}

	.nav-item:hover {
		background: var(--surface-hover);
		color: var(--text);
	}

	.nav-item.active {
		background: var(--accent-soft);
		color: var(--text);
	}

	.nav-label {
		flex: 1;
	}

	.nav-item.active .nav-label {
		color: var(--text);
	}

	.sidebar-foot {
		margin-top: auto;
		padding: 12px 0 6px;
		font-size: 12px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.modify-btn {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 6px 10px;
		border-radius: var(--radius-sm);
		border: 1px solid var(--border);
		background: var(--surface);
		color: var(--text-secondary);
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition:
			background 0.12s ease,
			border-color 0.12s ease,
			color 0.12s ease;
	}

	.modify-btn:hover {
		background: var(--surface-hover);
		border-color: var(--accent-border);
		color: var(--text);
	}

	.main {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.topbar {
		display: flex;
		align-items: center;
		gap: 16px;
		height: var(--topbar-h);
		padding: 0 20px;
		border-bottom: 1px solid var(--border);
		background: var(--bg);
		flex-shrink: 0;
	}

	.quick-search {
		position: relative;
		display: flex;
		align-items: center;
		gap: 8px;
		width: min(480px, 100%);
		padding: 7px 12px;
		border-radius: var(--radius-sm);
		border: 1px solid var(--border);
		background: var(--bg-elevated);
		color: var(--text-faint);
		transition: border-color 0.12s ease;
	}

	.quick-search:focus-within {
		border-color: var(--accent);
		box-shadow: 0 0 0 3px var(--accent-soft);
	}

	.quick-input {
		flex: 1;
		border: none;
		background: none;
		outline: none;
		color: var(--text);
		font-size: 13.5px;
		min-width: 0;
	}

	.quick-input::placeholder {
		color: var(--text-faint);
	}

	kbd {
		font-size: 11px;
		padding: 1px 6px;
		border-radius: 4px;
		border: 1px solid var(--border-strong);
		background: var(--surface);
		color: var(--text-faint);
		white-space: nowrap;
	}

	.topbar-actions {
		margin-left: auto;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.chip-btn {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 34px;
		height: 34px;
		border-radius: var(--radius-sm);
		border: 1px solid var(--border);
		color: var(--text-secondary);
		transition:
			background 0.12s ease,
			border-color 0.12s ease,
			color 0.12s ease;
	}

	.chip-btn:hover {
		background: var(--surface-hover);
		color: var(--text);
	}

	.chip-btn:has(.badge) {
		color: var(--accent);
	}

	.badge {
		position: absolute;
		top: -5px;
		right: -5px;
		min-width: 17px;
		height: 17px;
		padding: 0 4px;
		border-radius: 9px;
		background: var(--accent);
		color: #fff;
		font-size: 10.5px;
		font-weight: 650;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 0 0 2px var(--bg);
	}

	.badge.off {
		background: var(--text-faint);
	}

	.account-chip {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 5px 12px 5px 6px;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: var(--bg-elevated);
		color: var(--text-secondary);
		font-weight: 520;
		font-size: 13px;
		transition: border-color 0.12s ease, color 0.12s ease;
	}

	.account-chip:hover {
		border-color: var(--border-strong);
		color: var(--text);
	}

	.avatar {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: var(--accent-soft);
		color: var(--accent-hover);
		font-size: 12px;
		font-weight: 650;
	}

	.content {
		flex: 1;
		overflow-y: auto;
		position: relative;
	}

	.boot-splash {
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.boot-spinner {
		width: 26px;
		height: 26px;
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

	@media (max-width: 720px) {
		.sidebar {
			width: 60px;
		}

		.brand-name,
		.nav-label,
		.sidebar-foot {
			display: none;
		}

		.nav-item {
			justify-content: center;
		}

		.account-name {
			display: none;
		}
	}
</style>