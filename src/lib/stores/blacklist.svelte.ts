import { cacheGetJson, cacheSetJson } from '$lib/cache';
import { tagQueryPart } from '$lib/query';
import type { BlacklistEntry, GalleryListItem, Tag, TagRef } from '$lib/types';

const KEY = 'blacklist:v1';

let entries = $state<BlacklistEntry[]>([]);

function persist(): void {
	cacheSetJson(KEY, entries);
}

export async function loadBlacklist(): Promise<void> {
	const saved = await cacheGetJson<BlacklistEntry[]>(KEY);
	if (saved) entries = saved;
}

export function getBlacklist(): BlacklistEntry[] {
	return entries;
}

export function fromTag(tag: Tag): BlacklistEntry {
	return {
		id: tag.id,
		name: tag.name,
		type: tag.type,
		slug: tag.slug,
		count: tag.count,
		addedAt: Date.now(),
	};
}

export function fromRef(ref: TagRef): BlacklistEntry {
	return {
		id: ref.id,
		name: ref.name,
		type: ref.type,
		slug: ref.slug,
		addedAt: Date.now(),
	};
}

export function addTag(tag: Tag | TagRef): void {
	if (entries.some((e) => e.name === tag.name && e.type === tag.type)) return;
	entries = [fromTag(tag as Tag), ...entries];
	persist();
}

export function addTagByName(name: string, type = 'tag', id = 0): void {
	if (entries.some((e) => e.name === name && e.type === type)) return;
	entries = [{ id, name, type, slug: name.toLowerCase().replace(/\s+/g, '-'), addedAt: Date.now() }, ...entries];
	persist();
}

export function removeEntry(index: number): void {
	entries = entries.filter((_, i) => i !== index);
	persist();
}

export function removeByRef(ref: TagRef): void {
	entries = entries.filter((e) => !(e.name === ref.name && e.type === ref.type));
	persist();
}

export function clearBlacklist(): void {
	entries = [];
	persist();
}

export function matchesBlacklist(item: GalleryListItem): boolean {
	if (entries.length === 0) return false;
	const ids = item.tag_ids ?? [];
	const names = new Set(entries.map((e) => e.name.toLowerCase()));
	return ids.some((id) => entries.some((e) => e.id !== 0 && e.id === id)) || names.size > 0;
}

export function buildServerExcludes(): string {
	return entries
		.map((e) => tagQueryPart(e.type || 'tag', e.name, true))
		.join(' ');
}