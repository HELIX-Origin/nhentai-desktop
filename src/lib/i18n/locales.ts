export type LocaleCode =
	| 'en'
	| 'ja'
	| 'zh-Hans'
	| 'zh-Hant'
	| 'ko'
	| 'es'
	| 'fr'
	| 'de'
	| 'ru'
	| 'pt'
	| 'it'
	| 'th'
	| 'vi'
	| 'id'
	| 'pl'
	| 'nl'
	| 'tr'
	| 'ar';

export interface Locale {
	code: LocaleCode;
	name: string;
	nativeName: string;
}

export const locales: Locale[] = [
	{ code: 'en', name: 'English', nativeName: 'English' },
	{ code: 'ja', name: 'Japanese', nativeName: '日本語' },
	{ code: 'zh-Hans', name: 'Chinese (Simplified)', nativeName: '简体中文' },
	{ code: 'zh-Hant', name: 'Chinese (Traditional)', nativeName: '繁體中文' },
	{ code: 'ko', name: 'Korean', nativeName: '한국어' },
	{ code: 'es', name: 'Spanish', nativeName: 'Español' },
	{ code: 'fr', name: 'French', nativeName: 'Français' },
	{ code: 'de', name: 'German', nativeName: 'Deutsch' },
	{ code: 'ru', name: 'Russian', nativeName: 'Русский' },
	{ code: 'pt', name: 'Portuguese', nativeName: 'Português' },
	{ code: 'it', name: 'Italian', nativeName: 'Italiano' },
	{ code: 'th', name: 'Thai', nativeName: 'ไทย' },
	{ code: 'vi', name: 'Vietnamese', nativeName: 'Tiếng Việt' },
	{ code: 'id', name: 'Indonesian', nativeName: 'Bahasa Indonesia' },
	{ code: 'pl', name: 'Polish', nativeName: 'Polski' },
	{ code: 'nl', name: 'Dutch', nativeName: 'Nederlands' },
	{ code: 'tr', name: 'Turkish', nativeName: 'Türkçe' },
	{ code: 'ar', name: 'Arabic', nativeName: 'العربية' },
];

export const defaultLocale: LocaleCode = 'en';

export function isLocaleCode(value: string): value is LocaleCode {
	return locales.some((l) => l.code === value);
}
