import Redis from 'ioredis-mock';
import { backend } from '$lib/client';

const redis = new Redis();

const PREFIX = 'nhentai:';

function appKey(key: string): string {
	return PREFIX + key;
}

export async function cacheGet(key: string): Promise<string | null> {
	return redis.get(appKey(key));
}

export async function cacheSet(key: string, value: string): Promise<void> {
	await redis.set(appKey(key), value);
	backend.dbSet(appKey(key), value).catch(() => undefined);
}

export async function cacheDel(key: string): Promise<void> {
	await redis.del(appKey(key));
	backend.dbDel(appKey(key)).catch(() => undefined);
}

export async function cacheGetJson<T>(key: string): Promise<T | null> {
	const raw = await cacheGet(key);
	if (raw === null) return null;
	try {
		return JSON.parse(raw) as T;
	} catch {
		return null;
	}
}

export async function cacheSetJson(key: string, value: unknown): Promise<void> {
	await cacheSet(key, JSON.stringify(value));
}

export async function cacheInit(): Promise<void> {
	const rows = await backend.dbDump();
	for (const [key, value] of rows) {
		if (key.startsWith(PREFIX) || !key.includes(':')) {
			await redis.set(key, value);
		}
	}
}

export async function cacheFlush(): Promise<void> {
	const keys = await redis.keys('*');
	if (keys.length > 0) await redis.del(...keys);
	await backend.dbClear().catch(() => undefined);
}