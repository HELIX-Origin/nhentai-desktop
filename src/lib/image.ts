import { backend } from '$lib/client';

export const IMAGE_HOST = 'https://i.nhentai.net';
export const THUMB_HOST = 'https://t.nhentai.net';
export const AVATAR_HOST = 'https://static.nhentai.net';

function joinUrl(host: string, href: string | null | undefined): string {
	if (!href) return '';
	return /^https?:\/\//i.test(href) ? href : `${host}/${href.replace(/^\/+/, '')}`;
}

export function pagePath(path: string): string {
	return joinUrl(IMAGE_HOST, path);
}

export function thumbPath(path: string): string {
	return joinUrl(THUMB_HOST, path);
}

export function avatarUrl(href: string | null | undefined): string {
	return joinUrl(AVATAR_HOST, href);
}

const blobCache = new Map<string, string>();

export function blobUrlFor(href: string): string | undefined {
	return blobCache.get(href);
}

export function invalidateBlobUrl(href: string): void {
	const url = blobCache.get(href);
	if (url) {
		URL.revokeObjectURL(url);
		blobCache.delete(href);
	}
}

export function revokeAllBlobs(): void {
	for (const url of blobCache.values()) URL.revokeObjectURL(url);
	blobCache.clear();
}

export async function proxiedBlobUrl(href: string): Promise<string> {
	const existing = blobCache.get(href);
	if (existing) return existing;
	const bytes = await backend.proxyImage(href);
	const blob = new Blob([new Uint8Array(bytes)], { type: 'image/jpeg' });
	const url = URL.createObjectURL(blob);
	blobCache.set(href, url);
	return url;
}