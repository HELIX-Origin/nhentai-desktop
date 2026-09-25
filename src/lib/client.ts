import { invoke } from '@tauri-apps/api/core';
import type {
	ApiKeyStatus,
	AutoRefreshConfig,
	BlacklistListResponse,
	DownloadResponse,
	FavoriteResponse,
	GalleryDetail,
	GalleryList,
	GalleryListItem,
	Paginated,
	RelatedGalleries,
	ServiceStatus,
	Tag,
	UserMeResponse,
} from '$lib/types';

function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	return invoke<T>(cmd, args);
}

export const backend = {
	fetchNew: (page?: number, perPage?: number) =>
		call<GalleryList>('fetch_new', { page, perPage }),
	fetchPopular: () => call<GalleryListItem[]>('fetch_popular'),
	fetchTagged: (tagId: number, sort?: string, page?: number, perPage?: number) =>
		call<GalleryList>('fetch_tagged', { tagId, sort, page, perPage }),
	searchGalleries: (query: string, sort?: string, page?: number) =>
		call<GalleryList>('search_galleries', { query, sort, page }),
	fetchGallery: (id: number, include?: string) =>
		call<GalleryDetail>('fetch_gallery', { id, include }),
	relatedGalleries: (id: number) => call<RelatedGalleries>('related_galleries', { id }),
	fetchTagInfo: (tagType: string, slug: string) =>
		call<Tag>('fetch_tag_info', { tagType, slug }),
	fetchTagsByType: (tagType: string, sort?: string, page?: number, perPage?: number) =>
		call<Paginated<Tag>>('fetch_tags_by_type', { tagType, sort, page, perPage }),
	proxyImage: (url: string) => call<number[]>('proxy_image', { url }),

	dbGet: (key: string) => call<string | null>('db_get', { key }),
	dbSet: (key: string, value: string) => call<void>('db_set', { key, value }),
	dbDel: (key: string) => call<void>('db_del', { key }),
	dbDump: () => call<[string, string][]>('db_dump'),
	dbClear: () => call<void>('db_clear'),

	setApiKey: (key: string) => call<void>('set_api_key', { key }),
	getApiKeyStatus: () => call<ApiKeyStatus>('get_api_key_status'),
	clearApiKey: () => call<void>('clear_api_key'),
	verifyApiKey: () => call<UserMeResponse>('verify_api_key'),
	getCurrentUser: () => call<UserMeResponse>('get_current_user'),

	checkFavorite: (id: number) => call<FavoriteResponse>('check_favorite', { id }),
	addFavorite: (id: number) => call<FavoriteResponse>('add_favorite', { id }),
	removeFavorite: (id: number) => call<FavoriteResponse>('remove_favorite', { id }),
	fetchFavorites: (query?: string, page?: number) =>
		call<GalleryList>('fetch_favorites', { query, page }),

	fetchAccountBlacklist: () => call<BlacklistListResponse>('fetch_account_blacklist'),
	updateAccountBlacklist: (added: number[], removed: number[]) =>
		call<unknown>('update_account_blacklist', { added, removed }),

	downloadGallery: (id: number, format: 'zip' | 'cbz' | 'torrent' = 'zip') =>
		call<DownloadResponse>('download_gallery', { id, format }),

	serviceEnqueueDownload: (id: number, format: 'zip' | 'cbz' | 'torrent' = 'zip') =>
		call<number>('service_enqueue_download', { id, format }),
	serviceEnqueuePrefetch: (urls: string[]) =>
		call<number>('service_enqueue_prefetch', { urls }),
	serviceEnqueueMaintenance: () => call<number>('service_enqueue_maintenance'),
	serviceEnqueueSync: () => call<number>('service_enqueue_sync'),
	serviceStatus: () => call<ServiceStatus>('service_status'),
	serviceSetAutoRefresh: (enabled: boolean, intervalMinutes: number) =>
		call<void>('service_set_auto_refresh', { enabled, intervalMinutes }),
	serviceGetAutoRefresh: () => call<AutoRefreshConfig>('service_get_auto_refresh'),
	serviceGetDownloadsDir: () => call<string>('service_get_downloads_dir'),
	serviceSetDownloadsDir: (dir: string) =>
		call<void>('service_set_downloads_dir', { dir }),
	serviceResetDownloadsDir: () => call<void>('service_reset_downloads_dir'),
	openDownloadsFolder: () => call<void>('open_downloads_folder'),
};