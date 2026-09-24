import { cacheGetJson, cacheSetJson } from '$lib/cache';
import type { SettingsState } from '$lib/types';

const KEY = 'settings:v1';

const DEFAULT_SETTINGS: SettingsState = {
	theme: 'dark',
	density: 'cozy',
	blacklistEnabled: true,
	blacklistMode: 'hide',
	readerFit: 'width',
	readerRtl: false,
};

let state = $state<SettingsState>({ ...DEFAULT_SETTINGS });

function applyTheme(theme: 'dark'): void {
	document.documentElement.dataset.theme = theme;
}

function persist(): void {
	cacheSetJson(KEY, state);
}

export function getSettings(): SettingsState {
	return state;
}

export function updateSettings(patch: Partial<SettingsState>): void {
	Object.assign(state, patch);
	if (patch.theme) applyTheme(patch.theme);
	persist();
}

export async function loadSettings(): Promise<void> {
	const saved = await cacheGetJson<Partial<SettingsState>>(KEY);
	if (saved) {
		state = { ...DEFAULT_SETTINGS, ...saved };
	}
	applyTheme(state.theme);
}

export function settingsReady(): boolean {
	return document.documentElement.dataset.theme === state.theme;
}