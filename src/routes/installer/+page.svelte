<svelte:options runes={true} />

<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { page } from '$app/state';
	import { base } from '$app/paths';
	import { locale } from '$lib/stores/locale.svelte';
	import { locales } from '$lib/i18n';
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
	let launchAfter = $state(false);

	let removeUserData = $state(false);

	let inProgress = $state(false);
	let progressPct = $state(0);
	let progressStep = $state('Preparing files…');
	let result = $state<OperationResult | null>(null);
	let logDetails = $state<string[]>([]);
	let showLogs = $state(false);

	const closeWindow = () => getCurrentWindow().close();
	const minimizeWindow = () => getCurrentWindow().minimize();
	const maximizeWindow = () => getCurrentWindow().toggleMaximize();

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

	function formatBytes(bytes: number): string {
		const gb = bytes / (1024 * 1024 * 1024);
		return `${gb.toFixed(1)} GB`;
	}

	onMount(async () => {
		await locale.init();
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
			targetDir = info?.default_install_dir ?? 'C:\\Program Files\\NH Desktop';
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

	async function finishInstallation() {
		if (launchAfter) {
			try {
				await invoke<OperationResult>('installer_launch_app', { target_dir: targetDir });
			} catch {
				// Launch failure is non-fatal; still close the wizard.
			}
		}
		closeWindow();
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
	<title>NH Desktop Setup</title>
	<meta name="color-scheme" content="dark" />
</svelte:head>

<div class="installer-shell">
	<header
		class="titlebar"
		class:mac={isMac}
		role="presentation"
		onpointerdown={onBarPointerDown}
		ondblclick={onBarDoubleClick}
	>
		<div class="traffic" aria-label={locale.t('titlebar.closeTooltip')}>
			<button class="dot close" aria-label={locale.t('titlebar.closeTooltip')} onclick={closeWindow}></button>
			<button class="dot min" aria-label={locale.t('titlebar.minimizeTooltip')} onclick={minimizeWindow}></button>
			<button class="dot max" aria-label={locale.t('titlebar.maximizeTooltip')} onclick={maximizeWindow}></button>
		</div>
		<span class="tb-title">{locale.t('app.setup')}</span>
		<div class="spacer"></div>
	</header>

	{#if statusLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>{locale.t('common.loading')}</p>
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
					<span class="tab-text">{locale.t('installer.welcome')}</span>
				</button>
				<button
					class="tab-btn"
					class:active={installTab === 'destination'}
					onclick={() => {
						if (!inProgress && installTab !== 'complete') installTab = 'destination';
					}}
				>
					<span class="tab-num">2</span>
					<span class="tab-text">{locale.t('installer.location')}</span>
				</button>
				<button
					class="tab-btn"
					class:active={installTab === 'options'}
					onclick={() => {
						if (!inProgress && installTab !== 'complete') installTab = 'options';
					}}
				>
					<span class="tab-num">3</span>
					<span class="tab-text">{locale.t('installer.options')}</span>
				</button>
				<button
					class="tab-btn"
					class:active={installTab === 'installing' || installTab === 'complete'}
					disabled={installTab !== 'installing' && installTab !== 'complete'}
				>
					<span class="tab-num">4</span>
					<span class="tab-text">{locale.t('installer.install')}</span>
				</button>
			</nav>

			<main class="wizard-page">
				{#if installTab === 'welcome'}
					<div class="page-content hero-page">
						<h2>{locale.t('installer.welcomeTitle')}</h2>
						<p class="hero-desc">{locale.t('installer.welcomeDescription')}</p>
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
						<h3>{locale.t('installer.destinationTitle')}</h3>
						<p class="section-desc">{locale.t('installer.destinationDescription')}</p>

						<div class="path-card">
							<label class="path-label" for="target-path">{locale.t('installer.destinationFolder')}</label>
							<input id="target-path" type="text" class="path-input" bind:value={targetDir} />
						</div>

						<div class="space-info">
							<div class="space-row">
								<span>{locale.t('installer.spaceRequired')}</span>
								<strong>~128 MB</strong>
							</div>
							<div class="space-row">
								<span>{locale.t('installer.spaceAvailable')}</span>
								<strong class="space-ok">Plenty available</strong>
							</div>
						</div>
					</div>

				{:else if installTab === 'options'}
					<div class="page-content">
						<h3>{locale.t('installer.optionsTitle')}</h3>
						<p class="section-desc">{locale.t('installer.optionsDescription')}</p>

						<div class="options-group">
							<label class="check-option lang-option">
								<div class="opt-desc">
									<strong>{locale.t('installer.languageLabel')}</strong>
									<span>{locale.t('installer.languageDescription')}</span>
								</div>
								<select
									class="lang-select"
									value={locale.value}
									onchange={(e) => locale.set(e.currentTarget.value)}
									aria-label={locale.t('installer.languageLabel')}
								>
									{#each locales as l}
										<option value={l.code}>{l.nativeName} ({l.name})</option>
									{/each}
								</select>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={createDesktop} />
								<div class="opt-desc">
									<strong>{locale.t('installer.desktopShortcut')}</strong>
									<span>Place a quick-launch shortcut on your desktop</span>
								</div>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={createStartMenu} />
								<div class="opt-desc">
									<strong>{locale.t('installer.startMenuShortcut')}</strong>
									<span>Register in your application menu for quick search</span>
								</div>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={addToPath} />
								<div class="opt-desc">
									<strong>{locale.t('installer.addToPath')}</strong>
									<span>Allows running the <code>NH Desktop</code> command in terminal</span>
								</div>
							</label>

							<label class="check-option">
								<input type="checkbox" bind:checked={launchAfter} />
								<div class="opt-desc">
									<strong>{locale.t('installer.launchAfter')}</strong>
									<span>Open NH Desktop immediately after setup completes</span>
								</div>
							</label>
						</div>
					</div>

				{:else if installTab === 'installing'}
					<div class="page-content progress-page">
						<h3>{locale.t('installer.installingTitle')}</h3>
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
						<h3>{locale.t('installer.completeTitle')}</h3>
						<p class="complete-desc">
							{result?.message ?? locale.t('installer.completeDescription')}
						</p>
						<p class="launch-hint">{locale.t('installer.launchHint')}</p>
					</div>
				{/if}
			</main>

			<footer class="wizard-footer">
				{#if installTab === 'welcome'}
					<button class="btn secondary" onclick={closeWindow}>{locale.t('installer.cancel')}</button>
					<button class="btn primary" onclick={() => (installTab = 'destination')}>{locale.t('installer.next')} ›</button>
				{:else if installTab === 'destination'}
					<button class="btn secondary" onclick={() => (installTab = 'welcome')}>‹ {locale.t('installer.back')}</button>
					<button class="btn primary" onclick={() => (installTab = 'options')}>{locale.t('installer.next')} ›</button>
				{:else if installTab === 'options'}
					<button class="btn secondary" onclick={() => (installTab = 'destination')}>‹ {locale.t('installer.back')}</button>
					<button class="btn primary install" onclick={startInstallation}>{locale.t('installer.installBtn')}</button>
				{:else if installTab === 'installing'}
					<div class="spacer"></div>
					<button class="btn" disabled>{locale.t('installer.installingBtn')}</button>
				{:else if installTab === 'complete'}
					<div class="spacer"></div>
					<button class="btn primary" onclick={finishInstallation}>{locale.t('installer.finish')}</button>
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
					<span class="tab-text">{locale.t('installer.maintenance')}</span>
				</button>
				<button
					class="tab-btn"
					class:active={maintenanceTab === 'uninstall_options'}
					onclick={() => {
						if (!inProgress && maintenanceTab !== 'finished') maintenanceTab = 'uninstall_options';
					}}
				>
					<span class="tab-num">2</span>
					<span class="tab-text">{locale.t('installer.uninstallOptions')}</span>
				</button>
				<button
					class="tab-btn"
					class:active={maintenanceTab === 'removing' || maintenanceTab === 'finished'}
					disabled={maintenanceTab !== 'removing' && maintenanceTab !== 'finished'}
				>
					<span class="tab-num">3</span>
					<span class="tab-text">{locale.t('installer.complete')}</span>
				</button>
			</nav>

			<main class="wizard-page">
				{#if maintenanceTab === 'manage'}
					<div class="page-content">
						<h3>{locale.t('installer.manageTitle')}</h3>
						<p class="section-desc">
							NH Desktop is currently installed at:
							<code>{info?.default_install_dir}</code>
						</p>

						<div class="maintenance-cards">
							<button
								class="card-btn"
								onclick={() => ((mode = 'install'), (installTab = 'destination'))}
							>
								<div class="card-title">{locale.t('installer.reinstall')}</div>
								<div class="card-sub">Reinstall or upgrade the client to version {info?.current_version}</div>
							</button>

							<button class="card-btn" onclick={startInstallation}>
								<div class="card-title">{locale.t('installer.repairShortcuts')}</div>
								<div class="card-sub">Recreate missing Desktop and Start Menu application links</div>
							</button>

							<button class="card-btn danger" onclick={() => (maintenanceTab = 'uninstall_options')}>
								<div class="card-title">{locale.t('installer.uninstall')}</div>
								<div class="card-sub">Remove the application and registered system handlers</div>
							</button>
						</div>
					</div>

				{:else if maintenanceTab === 'uninstall_options'}
					<div class="page-content">
						<h3>{locale.t('installer.uninstallOptionsTitle')}</h3>
						<p class="section-desc">Choose how your user data is handled:</p>

						<div class="options-group">
							<label class="check-option">
								<input type="checkbox" bind:checked={removeUserData} />
								<div class="opt-desc">
									<strong>{locale.t('installer.removeUserData')}</strong>
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
						<h3>{locale.t('installer.removingTitle')}</h3>
						<p class="section-desc">{progressStep}</p>

						<div class="progress-bar-bg">
							<div class="progress-bar-fill danger" style="width: {progressPct}%"></div>
						</div>
					</div>

				{:else if maintenanceTab === 'finished'}
					<div class="page-content complete-page">
						<div class="success-icon">✓</div>
						<h3>{locale.t('installer.uninstallCompleteTitle')}</h3>
						<p class="complete-desc">
							{result?.message ?? locale.t('installer.uninstallCompleteDescription')}
						</p>
					</div>
				{/if}
			</main>

			<footer class="wizard-footer">
				{#if maintenanceTab === 'manage'}
					<button class="btn secondary" onclick={closeWindow}>{locale.t('installer.cancel')}</button>
				{:else if maintenanceTab === 'uninstall_options'}
					<button class="btn secondary" onclick={() => (maintenanceTab = 'manage')}>‹ {locale.t('installer.back')}</button>
					<button class="btn danger" onclick={startUninstallation}>{locale.t('installer.uninstallBtn')}</button>
				{:else if maintenanceTab === 'removing'}
					<div class="spacer"></div>
					<button class="btn" disabled>{locale.t('installer.removingBtn')}</button>
				{:else if maintenanceTab === 'finished'}
					<div class="spacer"></div>
					<button class="btn primary" onclick={closeWindow}>{locale.t('installer.close')}</button>
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

	.spacer {
		flex: 1;
	}

	/* Default (Windows/Linux): traffic lights on the right. */
	.titlebar .tb-title { order: 1; }
	.titlebar .spacer { order: 2; }
	.titlebar .traffic { order: 3; }
	.titlebar .traffic .dot.min { order: 1; }
	.titlebar .traffic .dot.max { order: 2; }
	.titlebar .traffic .dot.close { order: 3; }

	/* macOS: traffic lights on the left, close/min/max order. */
	.titlebar.mac .traffic { order: 0; margin-right: 2px; }
	.titlebar.mac .traffic .dot.close { order: 1; }
	.titlebar.mac .traffic .dot.min { order: 2; }
	.titlebar.mac .traffic .dot.max { order: 3; }
	.titlebar.mac .tb-title { order: 2; }
	.titlebar.mac .spacer { order: 3; }

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

	.lang-option {
		align-items: center;
		justify-content: space-between;
	}

	.lang-select {
		background: var(--bg);
		border: 1px solid var(--border-strong);
		color: var(--text);
		padding: 7px 10px;
		border-radius: var(--radius-sm);
		font-size: 13px;
		outline: none;
		cursor: pointer;
		flex-shrink: 0;
		max-width: 260px;
	}

	.lang-select:focus {
		border-color: var(--accent);
		box-shadow: 0 0 0 3px var(--accent-soft);
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