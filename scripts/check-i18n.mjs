import { readFile, readdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const i18nDir = join(__dirname, '..', 'src', 'lib', 'i18n');

function flatten(obj, prefix = '') {
	return Object.entries(obj).flatMap(([key, value]) =>
		typeof value === 'string'
			? [[`${prefix}${key}`, value]]
			: value && typeof value === 'object'
				? flatten(value, `${prefix}${key}.`)
				: [],
	);
}

const en = JSON.parse(await readFile(join(i18nDir, 'en.json'), 'utf8'));
const enEntries = new Map(flatten(en));

// Keys that are intentionally identical across locales (brand names, key names).
const allowedSame = new Set(['app.name', 'titlebar.searchShortcut', 'app.setup']);

const files = (await readdir(i18nDir)).filter((f) => f.endsWith('.json') && f !== 'en.json');

let failed = false;

if (files.length === 0) {
	console.error('No language packs found besides en.json.');
	process.exit(1);
}

for (const file of files.sort()) {
	const locale = file.replace(/\.json$/, '');
	const dict = JSON.parse(await readFile(join(i18nDir, file), 'utf8'));
	const entries = new Map(flatten(dict));

	const missing = [...enEntries.keys()].filter((key) => !entries.has(key));
	const empty = [...entries.entries()].filter(([key, value]) => {
		const source = enEntries.get(key);
		return !value.trim() || (value.trim() === source?.trim() && !allowedSame.has(key));
	});
	const extra = [...entries.keys()].filter((key) => !enEntries.has(key));

	if (missing.length || empty.length || extra.length) {
		failed = true;
	}

	const status = missing.length === 0 && empty.length === 0 && extra.length === 0 ? 'OK' : 'ISSUES';
	console.log(
		`${locale.padEnd(10)} ${String(entries.size).padStart(3)}/${enEntries.size} keys  ${status}`,
	);
	if (missing.length) console.log(`  missing (${missing.length}): ${missing.join(', ')}`);
	if (empty.length) console.log(`  untranslated (${empty.length}): ${empty.map(([k]) => k).join(', ')}`);
	if (extra.length) console.log(`  unknown keys (${extra.length}): ${extra.join(', ')}`);
}

process.exit(failed ? 1 : 0);
