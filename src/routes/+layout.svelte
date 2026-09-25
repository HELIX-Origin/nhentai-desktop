<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import Icon from '$lib/components/Icon.svelte';
	import { avatarUrl } from '$lib/image';
	import { loadSettings, getSettings } from '$lib/stores/settings.svelte';
	import { loadLibrary } from '$lib/stores/library.svelte';
	import { loadBlacklist, getBlacklist } from '$lib/stores/blacklist.svelte';
	import { initAccount, getAccountState } from '$lib/stores/account.svelte';
	import { initServiceStore } from '$lib/stores/service.svelte';
	import { cacheInit } from '$lib/cache';
	import { setTitlebarQuery } from '$lib/stores/titlebarSearch.svelte';
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
		{ href: '/downloads', label: 'Downloads', icon: 'download' },
		{ href: '/blacklist', label: 'Blacklist', icon: 'shield' },
		{ href: '/settings', label: 'Settings', icon: 'settings' },
	];

	const settings = getSettings();
	const blacklist = $derived(getBlacklist());
	const account = $derived(getAccountState());

	let quickQuery = $state('');
	let ready = $state(false);
	let avatarBroken = $state(false);

	$effect(() => {
		account.user;
		avatarBroken = false;
	});

	function onQuickSearch(event: Event) {
		event.preventDefault();
		const q = quickQuery.trim();
		const path = page.url.pathname;

		if (path === '/search') {
			goto(`/search?${new URLSearchParams({ q })}`);
		} else if (path === '/favorites' || path === '/history') {
			setTitlebarQuery(q);
		} else {
			if (!q) return;
			goto(`/search?${new URLSearchParams({ q })}`);
		}
	}

	const compact = () => getCurrentWindow().minimize();
	const zoom = () => getCurrentWindow().toggleMaximize();
	const quit = () => getCurrentWindow().close();

	const isMac = $derived(
		typeof navigator !== 'undefined' && /Mac|iPhone|iPad/.test(navigator.userAgent),
	);

	function onBarPointerDown(e: PointerEvent) {
		if (e.button !== 0) return;
		const t = e.target as HTMLElement | null;
		if (t?.closest('button, select, input, a, [role="menuitem"]')) return;
		e.preventDefault();
		getCurrentWindow().startDragging();
	}

	function onBarDoubleClick(e: MouseEvent) {
		const t = e.target as HTMLElement | null;
		if (t?.closest('button, select, input, a')) return;
		getCurrentWindow().toggleMaximize();
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
			initServiceStore(),
		]).finally(() => {
			ready = true;
		});
	});
</script>

<svelte:head>
	<title>NH Desktop</title>
	<meta name="color-scheme" content="dark" />
</svelte:head>

{#if isChildWindow}
	{@render children()}
{:else}
<div class="app">
	<header
		class="titlebar"
		class:mac={isMac}
		role="presentation"
		onpointerdown={onBarPointerDown}
		ondblclick={onBarDoubleClick}
	>
		<div class="traffic" aria-label="Window controls">
			<button class="dot close" aria-label="Close window (to tray)" onclick={quit}></button>
			<button class="dot min" aria-label="Minimize window" onclick={compact}></button>
			<button class="dot max" aria-label="Maximize window" onclick={zoom}></button>
		</div>
		<span class="tb-title">NH Desktop</span>
		<form class="quick-search" onsubmit={onQuickSearch} role="search">
			<Icon name="search" size={16} />
			<input
				class="quick-input"
				placeholder="Search galleries…"
				bind:value={quickQuery}
				aria-label="Quick search"
			/>
			<kbd>Enter</kbd>
		</form>
		<div class="bar-actions">
			<a class="chip-btn" href="/blacklist" title="Blacklisted tags" aria-label="Blacklisted tags">
				<Icon name="shield" size={16} />
				{#if blacklist.length > 0}
					<span class="badge" class:off={!settings.blacklistEnabled}>
						{blacklist.length}
					</span>
				{/if}
			</a>

			<a class="account-chip" href="/settings" title="Account & settings">
				{#if account.keyStatus.configured}
					{#if account.user && !avatarBroken}
						<img
							class="avatar"
							src={avatarUrl(account.user.avatar_url)}
							alt=""
							onerror={() => (avatarBroken = true)}
						/>
					{:else if account.user}
						<span class="avatar">{account.user.username[0]?.toUpperCase()}</span>
					{:else}
						<Icon name="user" size={16} />
					{/if}
					<span class="account-name">{account.user?.username ?? 'Connected'}</span>
				{:else}
					<Icon name="user" size={16} />
					<span class="account-name">Sign in</span>
				{/if}
			</a>
		</div>
	</header>

	<div class="shell">
	<aside class="sidebar">
		<div class="brand">
			<img class="brand-mark" src={`${base}/favicon.png`} alt="NH Desktop logo" />
			<span class="brand-name">NH Desktop</span>
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
			<span class="faint">v0.2.0</span>
		</div>
	</aside>

	<div class="main">
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
</div>
{/if}

<style>
	.app {
		display: flex;
		flex-direction: column;
		height: 100vh;
	}

	.titlebar {
		position: relative;
		z-index: 10;
		flex: 0 0 auto;
		height: 36px;
		display: flex;
		align-items: center;
		gap: 14px;
		padding: 0 12px;
		background: var(--bg-elevated);
		border-bottom: 1px solid var(--border);
		user-select: none;
		cursor: grab;
	}

	.titlebar:active {
		cursor: grabbing;
	}

	.traffic {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.dot {
		width: 13px;
		height: 13px;
		border-radius: 50%;
		border: none;
		padding: 0;
		cursor: pointer;
		position: relative;
	}

	.dot.close {
		background: #ff5f57;
	}

	.dot.min {
		background: #febc2e;
	}

	.dot.max {
		background: #28c840;
	}

	.dot:hover::after {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: rgba(0, 0, 0, 0.25);
	}

	.tb-title {
		font-size: 13px;
		font-weight: 600;
		letter-spacing: -0.01em;
		color: var(--text-secondary);
	}

	.shell {
		display: flex;
		flex: 1;
		min-height: 0;
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
		display: block;
		width: 30px;
		height: 30px;
		border-radius: 8px;
		object-fit: contain;
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

	.main {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	/* Default (Windows/Linux): title on the left, search centered, actions right before traffic. */
	.titlebar .tb-title { order: 1; }
	.titlebar .quick-search { order: 2; margin-left: auto; margin-right: auto; }
	.titlebar .bar-actions { order: 3; }
	.titlebar .traffic { order: 4; }
	.titlebar .traffic .dot.min { order: 1; }
	.titlebar .traffic .dot.max { order: 2; }
	.titlebar .traffic .dot.close { order: 3; }

	/* macOS: traffic lights on the left, title next, search centered, actions on the right. */
	.titlebar.mac .traffic { order: 0; margin-right: 2px; }
	.titlebar.mac .traffic .dot.close { order: 1; }
	.titlebar.mac .traffic .dot.min { order: 2; }
	.titlebar.mac .traffic .dot.max { order: 3; }
	.titlebar.mac .tb-title { order: 2; }
	.titlebar.mac .quick-search { order: 3; margin-left: auto; margin-right: auto; }
	.titlebar.mac .bar-actions { order: 4; }

	.quick-search {
		position: relative;
		display: flex;
		align-items: center;
		gap: 8px;
		width: min(480px, 100%);
		padding: 5px 12px;
		border-radius: var(--radius-sm);
		border: 1px solid var(--border);
		background: var(--bg);
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
		font-size: 13px;
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

	.bar-actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.chip-btn {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 28px;
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
		height: 28px;
		padding: 0 12px 0 5px;
		border-radius: 999px;
		border: 1px solid var(--border);
		background: var(--bg);
		color: var(--text-secondary);
		font-weight: 520;
		font-size: 12.5px;
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
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--accent-soft);
		color: var(--accent-hover);
		font-size: 11px;
		font-weight: 650;
		object-fit: cover;
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

		.account-name,
		.tb-title,
		.titlebar kbd {
			display: none;
		}

		.titlebar .quick-search {
			width: 100%;
			flex: 1;
		}
	}
</style>