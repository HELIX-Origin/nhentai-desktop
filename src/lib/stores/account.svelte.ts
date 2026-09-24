import { backend } from '$lib/client';
import type {
	ApiKeyStatus,
	BlacklistedTagResponse,
	UserMeResponse,
} from '$lib/types';

let keyStatus = $state<ApiKeyStatus>({ configured: false, prefix: null });
let user = $state<UserMeResponse | null>(null);
let accountFavorites = $state<Map<number, boolean>>(new Map());
let accountBlacklist = $state<BlacklistedTagResponse[]>([]);
let busy = $state(false);
let error = $state<string | null>(null);

export function getAccountState() {
	return {
		keyStatus,
		user,
		accountFavorites,
		accountBlacklist,
		busy,
		error,
	};
}

export function isAccountFavorite(id: number): boolean {
	return accountFavorites.get(id) ?? false;
}

export async function initAccount(): Promise<void> {
	try {
		keyStatus = await backend.getApiKeyStatus();
		if (keyStatus.configured) {
			user = await backend.getCurrentUser();
			await syncAccountBlacklist();
		}
	} catch (e) {
		error = String(e);
	}
}

export async function setApiKey(key: string): Promise<void> {
	busy = true;
	error = null;
	try {
		await backend.setApiKey(key.trim());
		keyStatus = await backend.getApiKeyStatus();
		user = await backend.verifyApiKey();
		await syncAccountBlacklist();
	} catch (e) {
		error = String(e);
		throw e;
	} finally {
		busy = false;
	}
}

export async function clearApiKey(): Promise<void> {
	busy = true;
	error = null;
	try {
		await backend.clearApiKey();
		keyStatus = { configured: false, prefix: null };
		user = null;
		accountBlacklist = [];
		accountFavorites = new Map();
	} finally {
		busy = false;
	}
}

export async function refreshFavorite(id: number): Promise<void> {
	if (!keyStatus.configured) return;
	try {
		const res = await backend.checkFavorite(id);
		accountFavorites.set(id, res.favorited);
	} catch {
		// account favorites are best-effort
	}
}

export async function toggleAccountFavorite(id: number): Promise<boolean | null> {
	if (!keyStatus.configured) return null;
	const current = accountFavorites.get(id) ?? false;
	try {
		const res = current
			? await backend.removeFavorite(id)
			: await backend.addFavorite(id);
		accountFavorites.set(id, res.favorited);
		return res.favorited;
	} catch (e) {
		error = String(e);
		return null;
	}
}

export async function syncAccountBlacklist(): Promise<void> {
	if (!keyStatus.configured) return;
	try {
		const res = await backend.fetchAccountBlacklist();
		accountBlacklist = res.tags;
	} catch (e) {
		error = String(e);
	}
}

export async function addAccountBlacklist(tagIds: number[]): Promise<void> {
	await backend.updateAccountBlacklist([...tagIds], []);
	await syncAccountBlacklist();
}

export async function removeAccountBlacklist(tagIds: number[]): Promise<void> {
	await backend.updateAccountBlacklist([], [...tagIds]);
	await syncAccountBlacklist();
}