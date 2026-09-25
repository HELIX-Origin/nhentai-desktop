import { backend } from '$lib/client';
import { cacheGetJson, cacheSetJson } from '$lib/cache';
import type { GalleryDetail, GalleryList, GalleryListItem, Paginated, RelatedGalleries, Tag } from '$lib/types';

const TTL_HOURS = 24;
export const GALLERIES_PER_PAGE = 28;

async function cached<T>(key: string, fetchFn: () => Promise<T>): Promise<T> {
	const hit = await cacheGetJson<T>(key);
	if (hit !== null) return hit;
	const data = await fetchFn();
	await cacheSetJson(key, data);
	return data;
}

function cacheKey(parts: (string | number | undefined)[]): string {
	return parts.filter(Boolean).join(':');
}

export const api = {
	newGalleries(page = 1, perPage = GALLERIES_PER_PAGE): Promise<GalleryList> {
		return cached(cacheKey(['cache:new', page, perPage]), () =>
			backend.fetchNew(page, perPage),
		);
	},

	popular(): Promise<GalleryListItem[]> {
		return cached('cache:popular', () => backend.fetchPopular());
	},

	tagged(tagId: number, sort = 'date', page = 1, perPage = GALLERIES_PER_PAGE): Promise<GalleryList> {
		return cached(cacheKey(['cache:tagged', tagId, sort, page, perPage]), () =>
			backend.fetchTagged(tagId, sort, page, perPage),
		);
	},

	search(query: string, sort = 'date', page = 1, ttlHours = TTL_HOURS): Promise<GalleryList> {
		return cached(cacheKey(['cache:search', query, sort, page, ttlHours]), () =>
			backend.searchGalleries(query, sort, page),
		);
	},

	gallery(id: number, include = 'favorite'): Promise<GalleryDetail> {
		return cached(cacheKey(['cache:gallery', id, include]), () =>
			backend.fetchGallery(id, include),
		);
	},

	related(id: number): Promise<RelatedGalleries> {
		return cached(cacheKey(['cache:related', id]), () => backend.relatedGalleries(id));
	},

	tagsByType(tagType: string, sort = 'popular', page = 1, perPage = 24): Promise<Paginated<Tag>> {
		return cached(cacheKey(['cache:tags', tagType, sort, page, perPage]), () =>
			backend.fetchTagsByType(tagType, sort, page, perPage),
		);
	},
};