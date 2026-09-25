import { cacheGetJson, cacheSetJson } from '$lib/cache';
import type { SettingsState, ThemePreference } from '$lib/types';

const KEY = 'settings:v1';

const DEFAULT_SETTINGS: SettingsState = {
	theme: 'system',
	density: 'cozy',
	blacklistEnabled: true,
	blacklistMode: 'hide',
	readerFit: 'width',
	readerRtl: false,
};

type ResolvedTheme = 'dark' | 'light';

let state = $state<SettingsState>({ ...DEFAULT_SETTINGS });

let darkQuery: MediaQueryList | null = null;
let onDarkChange: ((e: MediaQueryListEvent) => void) | null = null;

function systemResolved(): ResolvedTheme {
	if (typeof window === 'undefined') return 'dark';
	if (!darkQuery) darkQuery = window.matchMedia('(prefers-color-scheme: dark)');
	return darkQuery.matches ? 'dark' : 'light';
}

function resolveTheme(theme: ThemePreference): ResolvedTheme {
	return theme === 'system' ? systemResolved() : theme;
}

function watchSystem(): void {
	if (!darkQuery) darkQuery = window.matchMedia('(prefers-color-scheme: dark)');
	if (!onDarkChange) {
		onDarkChange = () => {
			if (state.theme === 'system') applyTheme('system');
		};
		darkQuery.addEventListener('change', onDarkChange);
	}
}

function applyTheme(theme: ThemePreference): void {
	document.documentElement.dataset.theme = resolveTheme(theme);
	if (theme === 'system') watchSystem();
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
	return document.documentElement.dataset.theme === resolveTheme(state.theme);
}