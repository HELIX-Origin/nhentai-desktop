import { t, type LocaleCode, defaultLocale, isLocaleCode } from '$lib/i18n';
import { cacheGet, cacheSet } from '$lib/cache';
import { invoke } from '@tauri-apps/api/core';

const LOCALE_KEY = 'settings:locale';

function createLocaleStore() {
	let value = $state<LocaleCode>(defaultLocale);
	let ready = $state(false);

	async function init(): Promise<void> {
		const cached = await cacheGet(LOCALE_KEY);
		if (cached && isLocaleCode(cached)) {
			value = cached;
			ready = true;
			return;
		}

		try {
			const system: string = await invoke('get_system_locale');
			if (isLocaleCode(system)) {
				value = system;
			} else if (system.startsWith('zh')) {
				// Simplified vs Traditional is tricky without region; default to Simplified.
				value = 'zh-Hans';
			} else {
				value = defaultLocale;
			}
		} catch {
			value = defaultLocale;
		}
		ready = true;
	}

	async function set(locale: string): Promise<void> {
		if (!isLocaleCode(locale)) return;
		value = locale;
		await cacheSet(LOCALE_KEY, locale);
	}

	return {
		get value() {
			return value;
		},
		get ready() {
			return ready;
		},
		init,
		set,
		t: (key: string) => t(key, value),
	};
}

export const locale = createLocaleStore();
