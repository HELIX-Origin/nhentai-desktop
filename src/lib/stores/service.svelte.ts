import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { backend } from '$lib/client';
import type {
	AutoRefreshConfig,
	ServiceEvent,
	ServiceJobKind,
	ServiceStatus,
} from '$lib/types';

export interface ServiceJobView {
	jobId: number;
	kind: ServiceJobKind;
	state: 'queued' | 'running' | 'finished' | 'failed';
	message?: string;
	error?: string;
	done?: number;
	total?: number | null;
	label?: string;
}

let jobs = $state<ServiceJobView[]>([]);
let lastRefreshAt = $state<number | null>(null);
let autoRefresh = $state<AutoRefreshConfig>({ enabled: false, intervalMinutes: 15 });

const KIND_LABELS: Record<ServiceJobKind, string> = {
	download: 'Download',
	prefetch: 'Image prefetch',
	maintenance: 'Maintenance',
	refresh: 'Popular refresh',
	sync: 'Account sync',
};

export function getServiceJobs(): ServiceJobView[] {
	return jobs;
}

export function getServiceAutoRefresh(): AutoRefreshConfig {
	return autoRefresh;
}

export function getServiceLastRefreshAt(): number | null {
	return lastRefreshAt;
}

export function serviceKindLabel(kind: ServiceJobKind): string {
	return KIND_LABELS[kind];
}

function upsertJob(jobId: number, patch: Partial<ServiceJobView>): void {
	const existing = jobs.findIndex((j) => j.jobId === jobId);
	if (existing >= 0) {
		jobs[existing] = { ...jobs[existing], ...patch };
	} else {
		jobs = [patch as ServiceJobView, ...jobs];
	}
}

function applyEvent(event: ServiceEvent): void {
	switch (event.state) {
		case 'queued':
			upsertJob(event.job_id, { jobId: event.job_id, kind: event.kind, state: 'queued' });
			break;
		case 'started':
			upsertJob(event.job_id, { jobId: event.job_id, kind: event.kind, state: 'running' });
			break;
		case 'progress':
			upsertJob(event.job_id, {
				jobId: event.job_id,
				kind: event.kind,
				state: 'running',
				done: event.done,
				total: event.total,
				label: event.label,
			});
			break;
		case 'finished':
			upsertJob(event.job_id, {
				jobId: event.job_id,
				kind: event.kind,
				state: 'finished',
				message: event.message,
			});
			break;
		case 'failed':
			upsertJob(event.job_id, {
				jobId: event.job_id,
				kind: event.kind,
				state: 'failed',
				error: event.error,
			});
			break;
	}
}

let unlistenJob: UnlistenFn | null = null;
let unlistenRefresh: UnlistenFn | null = null;

export async function initServiceStore(): Promise<void> {
	if (!unlistenJob) {
		unlistenJob = await listen<ServiceEvent>('service://job', (e) => {
			applyEvent(e.payload);
			jobs = jobs.slice(0, 50);
		});
	}
	if (!unlistenRefresh) {
		unlistenRefresh = await listen<string>('service://refresh', () => {
			lastRefreshAt = Date.now();
		});
	}
	autoRefresh = await backend.serviceGetAutoRefresh();
}

export async function refreshServiceStatus(): Promise<ServiceStatus> {
	const status = await backend.serviceStatus();
	lastRefreshAt = lastRefreshAt;
	return status;
}

export async function setServiceAutoRefresh(config: AutoRefreshConfig): Promise<void> {
	await backend.serviceSetAutoRefresh(config.enabled, config.intervalMinutes);
	autoRefresh = config;
}

export async function enqueueMaintenance(): Promise<number> {
	return backend.serviceEnqueueMaintenance();
}

export async function enqueueSync(): Promise<number> {
	return backend.serviceEnqueueSync();
}

export async function enqueueDownload(id: number, format: 'zip' | 'cbz' | 'torrent' = 'zip'): Promise<number> {
	return backend.serviceEnqueueDownload(id, format);
}

export async function prefetchImages(urls: string[]): Promise<number> {
	return backend.serviceEnqueuePrefetch(urls);
}