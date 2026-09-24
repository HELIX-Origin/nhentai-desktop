import { cacheGetJson, cacheSetJson } from '$lib/cache';
import { thumbPath } from '$lib/image';
import type { GalleryDetail, GalleryListItem, HistoryEntry } from '$lib/types';

const FAV_KEY = 'favorites:v1';
const HIST_KEY = 'history:v1';
const MAX_HISTORY = 200;

let favorites = $state<Map<number, GalleryListItem>>(new Map());
let history = $state<HistoryEntry[]>([]);

export function toGalleryListItem(g: GalleryDetail): GalleryListItem {
	return {
		id: g.id,
		media_id: g.media_id,
		english_title: g.title.english,
		japanese_title: g.title.japanese,
		thumbnail: thumbPath(g.thumbnail.path),
		thumbnail_width: g.thumbnail.width,
		thumbnail_height: g.thumbnail.height,
		num_pages: g.num_pages,
		num_favorites: g.num_favorites,
		tag_ids: g.tags.map((t) => t.id),
		blacklisted: null,
	};
}

function persistFavorites(): void {
	cacheSetJson(FAV_KEY, [...favorites.values()]);
}

function persistHistory(): void {
	cacheSetJson(HIST_KEY, history);
}

export async function loadLibrary(): Promise<void> {
	const fav = await cacheGetJson<GalleryListItem[]>(FAV_KEY);
	if (fav) favorites = new Map(fav.map((g) => [g.id, g]));
	const hist = await cacheGetJson<HistoryEntry[]>(HIST_KEY);
	if (hist) history = hist;
}

export function getFavorites(): GalleryListItem[] {
	return [...favorites.values()];
}

export function isFavorite(id: number): boolean {
	return favorites.has(id);
}

export function toggleFavorite(g: GalleryListItem): void {
	if (favorites.has(g.id)) {
		favorites.delete(g.id);
	} else {
		favorites.set(g.id, g);
	}
	persistFavorites();
}

export function removeFavorite(id: number): void {
	if (favorites.delete(id)) persistFavorites();
}

export function clearFavorites(): void {
	favorites = new Map();
	persistFavorites();
}

export function getHistory(): HistoryEntry[] {
	return history;
}

export function addToHistory(entry: HistoryEntry): void {
	const next = history.filter((h) => h.galleryId !== entry.galleryId);
	next.unshift(entry);
	history = next.slice(0, MAX_HISTORY);
	persistHistory();
}

export function clearHistory(): void {
	history = [];
	persistHistory();
}