import { backend } from '$lib/client';

export const IMAGE_HOST = 'https://i.nhentai.net';
export const THUMB_HOST = 'https://t.nhentai.net';

export function pagePath(path: string): string {
	return path.startsWith('http') ? path : IMAGE_HOST + path;
}

export function thumbPath(path: string): string {
	return path.startsWith('http') ? path : THUMB_HOST + path;
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