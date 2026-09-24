import type { FilterModel, SortOption } from '$lib/types';

const TYPE_PREFIX: Record<string, string | null> = {
	artist: 'artist',
	character: 'character',
	parody: 'parody',
	group: 'group',
	language: 'language',
	category: 'category',
	tag: null,
};

export function tagQueryPart(type: string, name: string, excluded: boolean): string {
	const prefix = TYPE_PREFIX[type] ?? 'tag';
	const fragment = `${prefix}:${JSON.stringify(name)}`;
	return excluded ? `-${fragment}` : fragment;
}

export function buildQuery(f: FilterModel): string {
	const parts: string[] = [];

	const trimmed = f.query.trim();
	if (trimmed) parts.push(trimmed);

	for (const tag of f.included) parts.push(tagQueryPart(tag.type, tag.name, false));
	for (const tag of f.excluded) parts.push(tagQueryPart(tag.type, tag.name, true));

	if (f.language) parts.push(`language:${f.language}`);
	if (f.category) parts.push(`category:${f.category}`);
	if (f.minPages !== undefined && f.minPages > 0) parts.push(`pages:>${f.minPages}`);
	if (f.maxPages !== undefined && f.maxPages > 0) parts.push(`pages:<${f.maxPages}`);

	return parts.join(' ').replace(/\s+/g, ' ').trim();
}

export const SORT_OPTIONS: { value: SortOption; label: string }[] = [
	{ value: 'date', label: 'Newest' },
	{ value: 'popular-today', label: 'Popular today' },
	{ value: 'popular-week', label: 'Popular this week' },
	{ value: 'popular-month', label: 'Popular this month' },
	{ value: 'popular', label: 'Popular all time' },
];

export interface FilterSummary {
	query: string;
	count: number;
}

export function summarizeFilters(f: FilterModel): FilterSummary {
	const count =
		f.included.length +
		f.excluded.length +
		(f.language ? 1 : 0) +
		(f.category ? 1 : 0) +
		(f.minPages !== undefined && f.minPages > 0 ? 1 : 0) +
		(f.maxPages !== undefined && f.maxPages > 0 ? 1 : 0);
	return { query: f.query, count };
}

export function cloneFilters(f: FilterModel): FilterModel {
	return {
		query: f.query,
		included: f.included.map((t) => ({ ...t })),
		excluded: f.excluded.map((t) => ({ ...t })),
		language: f.language,
		category: f.category,
		minPages: f.minPages,
		maxPages: f.maxPages,
		sort: f.sort,
	};
}