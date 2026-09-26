import en from './en.json';
import ja from './ja.json';
import zhHans from './zh-Hans.json';
import zhHant from './zh-Hant.json';
import { type LocaleCode, defaultLocale, isLocaleCode, locales } from './locales';

const dictionaries: Partial<Record<LocaleCode, Record<string, unknown>>> = {
	en,
	ja,
	'zh-Hans': zhHans,
	'zh-Hant': zhHant,
};

export type { LocaleCode } from './locales';
export { locales, defaultLocale, isLocaleCode } from './locales';

export function loadDictionary(locale: LocaleCode, dict: Record<string, unknown>): void {
	dictionaries[locale] = dict;
}

export function getDictionary(locale: LocaleCode): Record<string, unknown> {
	return dictionaries[locale] ?? dictionaries[defaultLocale] ?? en;
}

export function t(key: string, locale: LocaleCode = defaultLocale): string {
	const parts = key.split('.');
	const value = parts.reduce<unknown>((obj, part) => {
		if (obj && typeof obj === 'object' && part in obj) {
			return (obj as Record<string, unknown>)[part];
		}
		return undefined;
	}, getDictionary(locale));
	if (typeof value === 'string') return value;

	const fallback = parts.reduce<unknown>((obj, part) => {
		if (obj && typeof obj === 'object' && part in obj) {
			return (obj as Record<string, unknown>)[part];
		}
		return undefined;
	}, getDictionary(defaultLocale));
	return typeof fallback === 'string' ? fallback : key;
}
