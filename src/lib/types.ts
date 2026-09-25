export type TagKind = 'artist' | 'character' | 'tag' | 'parody' | 'language' | 'category' | 'group';

export interface Tag {
	id: number;
	type: string;
	name: string;
	slug: string;
	url: string;
	count: number;
	description?: string | null;
	is_community?: boolean | null;
}

export interface GalleryListItem {
	id: number;
	media_id: string;
	english_title: string;
	japanese_title?: string | null;
	thumbnail: string;
	thumbnail_width?: number | null;
	thumbnail_height?: number | null;
	num_pages?: number | null;
	num_favorites?: number | null;
	tag_ids: number[];
	blacklisted?: boolean | null;
}

export interface GalleryTitle {
	english: string;
	japanese: string | null;
	pretty: string;
}

export interface CoverInfo {
	path: string;
	width: number;
	height: number;
}

export interface PageInfo {
	number: number;
	path: string;
	width: number;
	height: number;
	thumbnail: string;
	thumbnail_width: number;
	thumbnail_height: number;
}

export interface GalleryDetail {
	id: number;
	media_id: string;
	title: GalleryTitle;
	cover: CoverInfo;
	thumbnail: CoverInfo;
	upload_date?: number | null;
	scanlator?: string | null;
	num_pages?: number | null;
	num_favorites?: number | null;
	tags: Tag[];
	pages?: PageInfo[] | null;
	is_favorited?: boolean | null;
	related?: GalleryListItem[] | null;
	comment_count?: number | null;
}

export interface Paginated<T> {
	result: T[];
	num_pages: number;
	per_page?: number | null;
	total?: number | null;
}

export type GalleryList = Paginated<GalleryListItem>;

export interface RelatedGalleries {
	result: GalleryListItem[];
}

export interface FavoriteResponse {
	favorited: boolean;
	num_favorites: number | null;
}

export interface UserMeResponse {
	id: number;
	username: string;
	slug: string;
	avatar_url: string;
	theme?: string | null;
	is_staff?: boolean | null;
	is_superuser?: boolean | null;
	about?: string | null;
	favorite_tags?: string | null;
	email?: string | null;
}

export interface BlacklistedTagResponse {
	id: number;
	type: string;
	name: string;
	slug: string;
	count: number;
}

export interface BlacklistListResponse {
	tags: BlacklistedTagResponse[];
	count: number;
}

export interface DownloadResponse {
	url: string;
	expires_at: number;
}

export interface ApiKeyStatus {
	configured: boolean;
	prefix: string | null;
}

export type SortOption = 'date' | 'popular' | 'popular-today' | 'popular-week' | 'popular-month';

export type BlacklistMode = 'hide' | 'blur';

export interface BlacklistEntry {
	id: number;
	name: string;
	type: string;
	slug: string;
	count?: number;
	addedAt: number;
}

export interface HistoryEntry {
	galleryId: number;
	mediaId: string;
	englishTitle: string;
	thumbnail: string;
	numPages?: number | null;
	visitedAt: number;
}

export interface SettingsState {
	theme: 'dark';
	density: 'cozy' | 'compact';
	blacklistEnabled: boolean;
	blacklistMode: BlacklistMode;
	readerFit: 'width' | 'height' | 'contain';
	readerRtl: boolean;
}

export interface TagRef {
	id: number;
	name: string;
	type: string;
	slug: string;
}

export interface FilterModel {
	query: string;
	included: TagRef[];
	excluded: TagRef[];
	language?: string;
	category?: string;
	minPages?: number;
	maxPages?: number;
	sort: SortOption;
}

export type ServiceJobKind = 'download' | 'prefetch' | 'maintenance' | 'refresh' | 'sync';

export type ServiceEvent =
	| { state: 'queued'; job_id: number; kind: ServiceJobKind }
	| { state: 'started'; job_id: number; kind: ServiceJobKind }
	| {
			state: 'progress';
			job_id: number;
			kind: ServiceJobKind;
			done: number;
			total: number | null;
			label: string;
	  }
	| { state: 'finished'; job_id: number; kind: ServiceJobKind; message: string }
	| { state: 'failed'; job_id: number; kind: ServiceJobKind; error: string };

export interface ServiceStatus {
	pending: number;
}

export interface AutoRefreshConfig {
	enabled: boolean;
	intervalMinutes: number;
}