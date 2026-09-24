<svelte:options runes={true} />

<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { page } from '$app/state';
	import '../../lib/design/base.css';

	interface InstallerStatus {
		is_installed: boolean;
		installed_version: string | null;
		current_version: string;
		default_install_dir: string;
		current_exe_path: string | null;
		os: string;
	}

	interface DiskSpaceInfo {
		available_bytes: number;
		required_bytes: number;
		has_sufficient_space: boolean;
	}

	interface OperationResult {
		success: boolean;
		message: string;
		details: string[];
	}

	type WizardMode = 'install' | 'maintenance';
	type InstallTab = 'welcome' | 'destination' | 'options' | 'installing' | 'complete';
	type MaintenanceTab = 'manage' | 'uninstall_options' | 'removing' | 'finished';

	let statusLoading = $state(true);
	let info = $state<InstallerStatus | null>(null);
	let mode = $state<WizardMode>('install');

	let installTab = $state<InstallTab>('welcome');
	let maintenanceTab = $state<MaintenanceTab>('manage');

	let targetDir = $state('');
	let createDesktop = $state(true);
	let createStartMenu = $state(true);
	let addToPath = $state(true);
	let launchAfter = $state(true);

	let removeUserData = $state(false);

	let inProgress = $state(false);
	let progressPct = $state(0);
	let progressStep = $state('Preparing files…');
	let result = $state<OperationResult | null>(null);
	let logDetails = $state<string[]>([]);
	let showLogs = $state(false);

	const closeWindow = () => getCurrentWindow().close();
	const minimizeWindow = () => getCurrentWindow().minimize();

	function onHeaderPointerDown(e: PointerEvent) {
		if (e.button !== 0) return;
		const t = e.target as HTMLElement | null;
		if (t?.closest('button, input, select, a')) return;
		getCurrentWindow().startDragging();
	}

	function formatBytes(bytes: number): string {
		const gb = bytes / (1024 * 1024 * 1024);
		return `${gb.toFixed(1)} GB`;
	}

	onMount(async () => {
		try {
			const detected = await invoke<InstallerStatus>('installer_status');
			info = detected;
			targetDir = detected.default_install_dir;

			const requestedMode = page.url.searchParams.get('mode');
			if (requestedMode === 'uninstall' || requestedMode === 'maintenance') {
				mode = 'maintenance';
			} else if (requestedMode === 'install') {
				mode = 'install';
			} else {
				mode = detected.is_installed ? 'maintenance' : 'install';
			}
		} catch {
			targetDir = info?.default_install_dir ?? 'C:\\Program Files\\nhentai';
		} finally {
			statusLoading = false;
		}
	});

	async function startInstallation() {
		installTab = 'installing';
		inProgress = true;
		progressPct = 15;
		progressStep = 'Validating target directory…';

		try {
			await new Promise((r) => setTimeout(r, 400));
			progressPct = 40;
			progressStep = 'Copying program binaries & resources…';

			const res = await invoke<OperationResult>('installer_install', {
				options: {
					target_dir: targetDir,
					create_desktop_shortcut: createDesktop,
					create_start_menu_shortcut: createStartMenu,
					add_to_path: addToPath,
					launch_after: launchAfter,
				},
			});

			progressPct = 85;
			progressStep = 'Configuring system shortcuts & associations…';
			await new Promise((r) => setTimeout(r, 300));

			progressPct = 100;
			progressStep = 'Installation complete!';
			result = res;
			logDetails = res.details;
			installTab = 'complete';
		} catch (err) {
			result = {
				success: false,
				message: String(err),
				details: [String(err)],
			};
			installTab = 'complete';
		} finally {
			inProgress = false;
		}
	}

	async function startUninstallation() {
		maintenanceTab = 'removing';
		inProgress = true;
		progressPct = 25;
		progressStep = 'Terminating running instances…';

		try {
			await new Promise((r) => setTimeout(r, 400));
			progressPct = 60;
			progressStep = 'Removing shortcuts and program files…';

			const res = await invoke<OperationResult>('installer_uninstall', {
				options: {
					remove_user_data: removeUserData,
				},
			});

			progressPct = 100;
			progressStep = 'Removal complete!';
			result = res;
			logDetails = res.details;
			maintenanceTab = 'finished';
		} catch (err) {
			result = {
				success: false,
				message: String(err),
				details: [String(err)],
			};
			maintenanceTab = 'finished';
		} finally {
			inProgress = false;
		}
	}
</script>

<svelte:head>
	<title>nhentai Setup</title>
	<meta name="color-scheme" content="dark" />
</svelte:head>

<div class="installer-shell">
	<header class="wizard-header" role="presentation" onpointerdown={onHeaderPointerDown}>
		<div class="app-badge">
			<span class="app-icon">n</span>
			<span class="app-title">nhentai Setup</span>
			{#if info}
				<span class="version-tag">v{info.current_version}</span>
			{/if}
		</div>
		<div class="window-controls">
			<button class="win-btn" onclick={minimizeWindow} aria-label="Minimize">─</button>
			<button class="win-btn close" onclick={closeWindow} aria-label="Close">✕</button>
		</div>
	</header>

	{#if statusLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Initializing setup wizard…</p>
		</div>
	{:else if mode === 'install'}
		<div class="wizard-body">
			<nav class="tab-strip" aria-label="Installer navigation">
				<button
					class="tab-btn"
					class:active={installTab === 'welcome'}
					onclick={() => {
						if (!inProgress && installTab !== 'complete') installTab = 'welcome';
					}}
				>
					<span class="tab-num">1</span>
					<span class="tab-text">Welcome</span>
				</button>
				<button
					class="tab-btn"
					class:active={installTab === 'destination'}
					onclick={() => {
						if (!inProgress && installTab !== 'complete') installTab = 'destination';
					}}
				>
					<span class="tab-num">2</span>
					<span class="tab-text">Location</span>
				</button>
				<button
					class="tab-btn"
					class:active={installTab === 'options'}
					onclick={() => {
						if (!inProgress && installTab !== 'complete') installTab = 'options';
					}}
				>
					<span class="tab-num">3</span>
					<span class="tab-text">Options</span>
				</button>
				<button
					class="tab-btn"
					class:active={installTab === 'installing' || installTab === 'complete'}
					disabled={installTab !== 'installing' && installTab !== 'complete'}
				>
					<span class="tab-num">4</span>
					<span class="tab-text">Install</span>
				</button>
			</nav>

			<main class="wizard-page">
				{#if installTab === 'welcome'}
					<div class="page-content hero-page">
						<h2>Welcome to nhentai</h2>
						<p class="hero-desc">
							A lightweight, modern desktop client for nhentai.net. Browse the full
							catalog with fast search, powerful filtering, and a global blacklist
							that actually works.
						</p>
						<div class="feature-list">
							<div class="feat-item">
								<span class="feat-dot">✦</span>
								<div>
									<strong>Global Blacklist:</strong> Hide or blur any tag, artist,
									or parody across the entire app with one toggle.
								</div>
							</div>
							<div class="feat-item">
								<span class="feat-dot">✦</span>
								<div>
									<strong>Powerful Search:</strong> Filter and sort the large
									local catalog faster than the site ever could.
								</div>
							</div>
							<div class="feat-item">
								<span class="feat-dot">✦</span>
								<div>
									<strong>Cross-Platform:</strong> Native performance on Windows,
									macOS, and Linux.
								</div>
							</div>
						</div>
					</div>

				{:else if installTab === 'destination'}
					<div class="page-content">
						<h3>Choose Install Location</h3>
						<p class="section-desc">Setup will install nhentai into the following directory:</p>

						<div class="path-card">
							<label class="path-label" for="target-path">Destination Folder:</label>
							<input id="target-path" type="text" class="path-input" bind:value={targetDir} />
						</div>

						<div class="space-info">
							<div class="space-row">
								<span>Space required:</span>
								<strong>~128 MB</strong>
							</div>
							<div class="space-row">
								<span>Space available:</span>
								<strong class="space-ok">Plenty available</strong>
							</div>
						</div>
					</div>

				{:else if installTab === 'options'}
					<div class="page-content">
						<h3>Select Additional Tasks</h3>
						<p class="section-desc">Configure desktop integration and launch preferences:</p>

						<div class="options-group">
							<label class="check-option">
								<input type="checkbox" bind:checked={createDesktop} />
								<div class="opt-desc">
									<strong>Create a Desktop Shortcut</strong>
									<span>Place a quick-launch shortcut on your desktop</span>
								</div>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={createStartMenu} />
								<div class="opt-desc">
									<strong>Create a Start Menu Shortcut</strong>
									<span>Register in your application menu for quick search</span>
								</div>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={addToPath} />
								<div class="opt-desc">
									<strong>Enable CLI Access (PATH)</strong>
									<span>Allows running the <code>nhentai</code> command in terminal</span>
								</div>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={launchAfter} />
								<div class="opt-desc">
									<strong>Launch after finish</strong>
									<span>Open nhentai immediately after setup completes</span>
								</div>
							</label>
						</div>
					</div>

				{:else if installTab === 'installing'}
					<div class="page-content progress-page">
						<h3>Installing nhentai…</h3>
						<p class="section-desc">{progressStep}</p>

						<div class="progress-bar-bg">
							<div class="progress-bar-fill" style="width: {progressPct}%"></div>
						</div>

						<div class="log-toggle-row">
							<button class="link-btn" onclick={() => (showLogs = !showLogs)}>
								{showLogs ? 'Hide details' : 'Show details'}
							</button>
						</div>

						{#if showLogs}
							<div class="install-logs">
								{#each logDetails as item}
									<div>{item}</div>
								{/each}
							</div>
						{/if}
					</div>

				{:else if installTab === 'complete'}
					<div class="page-content complete-page">
						<div class="success-icon">✓</div>
						<h3>Installation Completed</h3>
						<p class="complete-desc">
							{result?.message ?? 'nhentai has been successfully installed on your computer.'}
						</p>
						<p class="launch-hint">Click Finish to exit setup and begin browsing the catalog.</p>
					</div>
				{/if}
			</main>

			<footer class="wizard-footer">
				{#if installTab === 'welcome'}
					<button class="btn secondary" onclick={closeWindow}>Cancel</button>
					<button class="btn primary" onclick={() => (installTab = 'destination')}>Next ›</button>
				{:else if installTab === 'destination'}
					<button class="btn secondary" onclick={() => (installTab = 'welcome')}>‹ Back</button>
					<button class="btn primary" onclick={() => (installTab = 'options')}>Next ›</button>
				{:else if installTab === 'options'}
					<button class="btn secondary" onclick={() => (installTab = 'destination')}>‹ Back</button>
					<button class="btn primary install" onclick={startInstallation}>Install</button>
				{:else if installTab === 'installing'}
					<div class="spacer"></div>
					<button class="btn" disabled>Installing…</button>
				{:else if installTab === 'complete'}
					<div class="spacer"></div>
					<button class="btn primary" onclick={closeWindow}>Finish</button>
				{/if}
			</footer>
		</div>

	{:else}
		<div class="wizard-body">
			<nav class="tab-strip" aria-label="Maintenance navigation">
				<button
					class="tab-btn"
					class:active={maintenanceTab === 'manage'}
					onclick={() => {
						if (!inProgress && maintenanceTab !== 'finished') maintenanceTab = 'manage';
					}}
				>
					<span class="tab-num">1</span>
					<span class="tab-text">Maintenance</span>
				</button>
				<button
					class="tab-btn"
					class:active={maintenanceTab === 'uninstall_options'}
					onclick={() => {
						if (!inProgress && maintenanceTab !== 'finished') maintenanceTab = 'uninstall_options';
					}}
				>
					<span class="tab-num">2</span>
					<span class="tab-text">Options</span>
				</button>
				<button
					class="tab-btn"
					class:active={maintenanceTab === 'removing' || maintenanceTab === 'finished'}
					disabled={maintenanceTab !== 'removing' && maintenanceTab !== 'finished'}
				>
					<span class="tab-num">3</span>
					<span class="tab-text">Complete</span>
				</button>
			</nav>

			<main class="wizard-page">
				{#if maintenanceTab === 'manage'}
					<div class="page-content">
						<h3>Manage Installation</h3>
						<p class="section-desc">
							nhentai is currently installed at:
							<code>{info?.default_install_dir}</code>
						</p>

						<div class="maintenance-cards">
							<button
								class="card-btn"
								onclick={() => ((mode = 'install'), (installTab = 'destination'))}
							>
								<div class="card-title">Reinstall / Update</div>
								<div class="card-sub">Reinstall or upgrade the client to version {info?.current_version}</div>
							</button>

							<button class="card-btn" onclick={startInstallation}>
								<div class="card-title">Repair Shortcuts</div>
								<div class="card-sub">Recreate missing Desktop and Start Menu application links</div>
							</button>

							<button class="card-btn danger" onclick={() => (maintenanceTab = 'uninstall_options')}>
								<div class="card-title">Uninstall nhentai</div>
								<div class="card-sub">Remove the application and registered system handlers</div>
							</button>
						</div>
					</div>

				{:else if maintenanceTab === 'uninstall_options'}
					<div class="page-content">
						<h3>Uninstall Options</h3>
						<p class="section-desc">Choose how your user data is handled:</p>

						<div class="options-group">
							<label class="check-option">
								<input type="checkbox" bind:checked={removeUserData} />
								<div class="opt-desc">
									<strong>Delete all library and configuration data</strong>
									<span>Removes cached artwork, the local database, and configuration settings.</span>
								</div>
							</label>
						</div>

						<div class="warn-box">
							Your account's server-side favorites and blacklist are unaffected — only
							locally stored data is removed.
						</div>
					</div>

				{:else if maintenanceTab === 'removing'}
					<div class="page-content progress-page">
						<h3>Uninstalling nhentai…</h3>
						<p class="section-desc">{progressStep}</p>

						<div class="progress-bar-bg">
							<div class="progress-bar-fill danger" style="width: {progressPct}%"></div>
						</div>
					</div>

				{:else if maintenanceTab === 'finished'}
					<div class="page-content complete-page">
						<div class="success-icon">✓</div>
						<h3>Uninstallation Completed</h3>
						<p class="complete-desc">
							{result?.message ?? 'nhentai has been successfully removed from your computer.'}
						</p>
					</div>
				{/if}
			</main>

			<footer class="wizard-footer">
				{#if maintenanceTab === 'manage'}
					<button class="btn secondary" onclick={closeWindow}>Cancel</button>
				{:else if maintenanceTab === 'uninstall_options'}
					<button class="btn secondary" onclick={() => (maintenanceTab = 'manage')}>‹ Back</button>
					<button class="btn danger" onclick={startUninstallation}>Uninstall Now</button>
				{:else if maintenanceTab === 'removing'}
					<div class="spacer"></div>
					<button class="btn" disabled>Removing…</button>
				{:else if maintenanceTab === 'finished'}
					<div class="spacer"></div>
					<button class="btn primary" onclick={closeWindow}>Close</button>
				{/if}
			</footer>
		</div>
	{/if}
</div>

<style>
	:global(body) {
		margin: 0;
		user-select: none;
		overflow: hidden;
		font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.installer-shell {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: var(--bg);
		color: var(--text);
	}

	.wizard-header {
		height: 38px;
		background: var(--bg-elevated);
		border-bottom: 1px solid var(--border);
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 12px;
		cursor: grab;
	}

	.app-badge {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.app-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		border-radius: 5px;
		background: var(--accent);
		color: #fff;
		font-size: 12px;
		font-weight: 700;
	}

	.app-title {
		font-size: 13px;
		font-weight: 600;
	}

	.version-tag {
		font-size: 11px;
		color: var(--text-faint);
		background: var(--surface);
		padding: 2px 6px;
		border-radius: 4px;
	}

	.window-controls {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.win-btn {
		background: transparent;
		border: none;
		color: var(--text-faint);
		font-size: 12px;
		width: 28px;
		height: 24px;
		cursor: pointer;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.win-btn:hover {
		background: var(--surface-hover);
		color: var(--text);
	}

	.win-btn.close:hover {
		background: var(--danger);
		color: #fff;
	}

	.wizard-body {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.tab-strip {
		height: 48px;
		background: var(--bg-elevated);
		border-bottom: 1px solid var(--border);
		display: flex;
		padding: 0 16px;
		gap: 8px;
	}

	.tab-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--text-faint);
		font-size: 13px;
		font-weight: 500;
		padding: 0 12px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab-btn.active {
		color: var(--accent-hover);
		border-bottom-color: var(--accent);
	}

	.tab-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.tab-num {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--surface);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 11px;
		font-weight: 700;
	}

	.tab-btn.active .tab-num {
		background: var(--accent);
		color: #fff;
	}

	.wizard-page {
		flex: 1;
		padding: 24px 32px;
		overflow-y: auto;
	}

	.page-content h2,
	.page-content h3 {
		margin: 0 0 8px;
		font-weight: 600;
	}

	.section-desc {
		color: var(--text-secondary);
		font-size: 13px;
		margin: 0 0 20px;
	}

	.hero-page {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.hero-desc {
		color: var(--text-secondary);
		font-size: 14px;
		line-height: 1.5;
		margin: 0 0 12px;
	}

	.feature-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		padding: 16px;
	}

	.feat-item {
		display: flex;
		gap: 10px;
		font-size: 13px;
		line-height: 1.4;
	}

	.feat-dot {
		color: var(--accent);
	}

	.path-card {
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		padding: 14px;
		margin-bottom: 16px;
	}

	.path-label {
		display: block;
		font-size: 12px;
		font-weight: 600;
		color: var(--text-secondary);
		margin-bottom: 6px;
	}

	.path-input {
		width: 100%;
		box-sizing: border-box;
		background: var(--bg);
		border: 1px solid var(--border-strong);
		color: var(--text);
		padding: 8px 12px;
		border-radius: var(--radius-sm);
		font-size: 13px;
		outline: none;
	}

	.path-input:focus {
		border-color: var(--accent);
		box-shadow: 0 0 0 3px var(--accent-soft);
	}

	.space-info {
		font-size: 12px;
		color: var(--text-secondary);
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.space-row {
		display: flex;
		justify-content: space-between;
	}

	.space-ok {
		color: var(--success);
	}

	.options-group {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.check-option {
		display: flex;
		gap: 12px;
		align-items: flex-start;
		padding: 10px 14px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-sm);
		cursor: pointer;
	}

	.check-option input {
		margin-top: 3px;
		accent-color: var(--accent);
	}

	.opt-desc {
		display: flex;
		flex-direction: column;
		gap: 2px;
		font-size: 13px;
	}

	.opt-desc span {
		font-size: 11px;
		color: var(--text-secondary);
	}

	.progress-page {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	.progress-bar-bg {
		height: 10px;
		background: var(--surface);
		border-radius: 5px;
		overflow: hidden;
		margin-bottom: 12px;
	}

	.progress-bar-fill {
		height: 100%;
		background: var(--accent);
		transition: width 0.3s ease;
	}

	.progress-bar-fill.danger {
		background: var(--danger);
	}

	.log-toggle-row {
		display: flex;
		justify-content: flex-end;
	}

	.link-btn {
		background: transparent;
		border: none;
		color: var(--accent-hover);
		font-size: 12px;
		cursor: pointer;
		text-decoration: underline;
	}

	.install-logs {
		margin-top: 12px;
		background: var(--bg-elevated);
		border: 1px solid var(--border);
		border-radius: var(--radius-sm);
		padding: 8px 12px;
		max-height: 140px;
		overflow-y: auto;
		font-family: ui-monospace, 'Cascadia Code', Consolas, monospace;
		font-size: 11px;
		color: var(--text-secondary);
		line-height: 1.4;
	}

	.complete-page {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding-top: 24px;
	}

	.success-icon {
		width: 52px;
		height: 52px;
		border-radius: 50%;
		background: var(--success-soft);
		color: var(--success);
		font-size: 26px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 16px;
		border: 2px solid var(--success);
	}

	.complete-desc {
		color: var(--text-secondary);
		font-size: 14px;
		max-width: 480px;
		margin: 0 0 8px;
	}

	.launch-hint {
		color: var(--accent-hover);
		font-size: 12px;
		margin: 0;
	}

	.maintenance-cards {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-top: 16px;
	}

	.card-btn {
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		padding: 16px;
		text-align: left;
		color: var(--text);
		cursor: pointer;
		transition: all 0.15s;
	}

	.card-btn:hover {
		border-color: var(--accent-border);
		background: var(--surface-hover);
	}

	.card-btn.danger:hover {
		border-color: var(--danger);
		background: var(--danger-soft);
	}

	.card-title {
		font-weight: 600;
		font-size: 14px;
		margin-bottom: 4px;
	}

	.card-sub {
		font-size: 12px;
		color: var(--text-secondary);
	}

	code {
		font-family: ui-monospace, 'Cascadia Code', Consolas, monospace;
		font-size: 12px;
		background: var(--surface);
		padding: 1px 5px;
		border-radius: 4px;
		border: 1px solid var(--border);
	}

	.warn-box {
		margin-top: 16px;
		padding: 12px;
		border-radius: var(--radius-sm);
		background: rgba(246, 181, 69, 0.1);
		border: 1px solid rgba(246, 181, 69, 0.4);
		font-size: 12px;
		color: var(--warning);
	}

	.wizard-footer {
		height: 56px;
		background: var(--bg-elevated);
		border-top: 1px solid var(--border);
		display: flex;
		align-items: center;
		justify-content: flex-end;
		padding: 0 24px;
		gap: 12px;
	}

	.btn {
		padding: 8px 18px;
		font-size: 13px;
		font-weight: 600;
		border-radius: var(--radius-sm);
		cursor: pointer;
		border: 1px solid transparent;
	}

	.btn.primary {
		background: var(--accent);
		color: #fff;
	}

	.btn.primary:hover:not(:disabled) {
		background: var(--accent-hover);
	}

	.btn.primary.install {
		background: var(--accent);
	}

	.btn.primary.install:hover:not(:disabled) {
		background: var(--accent-hover);
	}

	.btn.secondary {
		background: var(--surface);
		color: var(--text-secondary);
		border-color: var(--border-strong);
	}

	.btn.secondary:hover:not(:disabled) {
		background: var(--surface-hover);
		color: var(--text);
	}

	.btn.danger {
		background: var(--danger);
		color: #fff;
	}

	.btn.danger:hover:not(:disabled) {
		background: #ff6e81;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spacer {
		flex: 1;
	}

	.loading-state {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		color: var(--text-secondary);
		font-size: 13px;
	}

	.spinner {
		width: 28px;
		height: 28px;
		border: 3px solid var(--border-strong);
		border-top-color: var(--accent);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>