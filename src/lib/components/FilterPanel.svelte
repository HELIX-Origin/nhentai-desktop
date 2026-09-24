<script lang="ts">
	import { SORT_OPTIONS } from '$lib/query';
	import type { FilterModel, TagRef } from '$lib/types';
	import Icon from './Icon.svelte';
	import TagChip from './TagChip.svelte';

	let {
		model,
		onupdate,
	}: {
		model: FilterModel;
		onupdate: (m: FilterModel) => void;
	} = $props();

	const LANGUAGES = [
		'english',
		'japanese',
		'chinese',
		'vietnamese',
		'korean',
		'spanish',
		'french',
		'german',
		'italian',
		'portuguese',
		'russian',
		'polish',
		'other',
	];

	const CATEGORIES = [
		'doujinshi',
		'manga',
		'artistcg',
		'gamecg',
		'western',
		'non-h',
		'imageset',
		'cosplay',
		'asianporn',
		'misc',
	];

	let newTagName = $state('');
	let newTagType = $state('tag');

	function update(m: FilterModel) {
		onupdate(m);
	}

	function removeTag(tags: TagRef[], ref: TagRef): TagRef[] {
		return tags.filter((t) => !(t.name === ref.name && t.type === ref.type));
	}

	function addTag(excluded: boolean) {
		const name = newTagName.trim();
		if (!name) return;
		const ref: TagRef = { id: 0, name, type: newTagType, slug: name.toLowerCase().replace(/\s+/g, '-') };
		if (excluded) {
			update({ ...model, excluded: [...model.excluded, ref] });
		} else {
			update({ ...model, included: [...model.included, ref] });
		}
		newTagName = '';
	}

	function reset() {
		onupdate({
			query: model.query,
			included: [],
			excluded: [],
			language: undefined,
			category: undefined,
			minPages: undefined,
			maxPages: undefined,
			sort: 'date',
		});
	}
</script>

<div class="filters">
	<section class="group">
		<h4>Sort</h4>
		<div class="seg">
			{#each SORT_OPTIONS as opt}
				<button class:on={model.sort === opt.value} onclick={() => update({ ...model, sort: opt.value })}>
					{opt.label}
				</button>
			{/each}
		</div>
	</section>

	{#if model.included.length > 0}
		<section class="group">
			<h4>Included tags</h4>
			<div class="chips">
				{#each model.included as t}
					<TagChip name={t.name} type={t.type} active onclick={() => update({ ...model, included: removeTag(model.included, t) })} />
				{/each}
			</div>
		</section>
	{/if}

	{#if model.excluded.length > 0}
		<section class="group">
			<h4>Excluded tags</h4>
			<div class="chips">
				{#each model.excluded as t}
					<TagChip name={t.name} type={t.type} onclick={() => update({ ...model, excluded: removeTag(model.excluded, t) })} />
				{/each}
			</div>
		</section>
	{/if}

	<section class="group">
		<h4>Add tag filter</h4>
		<div class="add-row">
			<input class="input" placeholder="Tag name…" bind:value={newTagName} aria-label="Tag name to filter" />
			<select class="select type" bind:value={newTagType} aria-label="Tag type">
				<option value="tag">Tag</option>
				<option value="artist">Artist</option>
				<option value="character">Character</option>
				<option value="parody">Parody</option>
				<option value="group">Group</option>
				<option value="language">Language</option>
				<option value="category">Category</option>
			</select>
		</div>
		<div class="add-actions">
			<button class="btn" onclick={() => addTag(false)} disabled={!newTagName.trim()}>
				<Icon name="plus" size={14} />
				Include
			</button>
			<button class="btn" onclick={() => addTag(true)} disabled={!newTagName.trim()}>
				<Icon name="minus" size={14} />
				Exclude
			</button>
		</div>
	</section>

	<section class="group">
		<h4>Language</h4>
		<select class="select" value={model.language ?? ''} onchange={(e) => update({ ...model, language: e.currentTarget.value || undefined })}>
			<option value="">Any</option>
			{#each LANGUAGES as lang}
				<option value={lang} selected={model.language === lang}>{lang}</option>
			{/each}
		</select>
	</section>

	<section class="group">
		<h4>Category</h4>
		<select class="select" value={model.category ?? ''} onchange={(e) => update({ ...model, category: e.currentTarget.value || undefined })}>
			<option value="">Any</option>
			{#each CATEGORIES as cat}
				<option value={cat} selected={model.category === cat}>{cat}</option>
			{/each}
		</select>
	</section>

	<section class="group row-group">
		<h4>Page count</h4>
		<div class="range-row">
			<input
				class="input"
				type="number"
				placeholder="Min"
				min="1"
				value={model.minPages ?? ''}
				oninput={(e) => update({ ...model, minPages: e.currentTarget.value ? Number(e.currentTarget.value) : undefined })}
				aria-label="Minimum pages"
			/>
			<span class="faint">to</span>
			<input
				class="input"
				type="number"
				placeholder="Max"
				min="1"
				value={model.maxPages ?? ''}
				oninput={(e) => update({ ...model, maxPages: e.currentTarget.value ? Number(e.currentTarget.value) : undefined })}
				aria-label="Maximum pages"
			/>
		</div>
	</section>

	<div class="foot">
		<button class="btn btn-ghost faint" onclick={reset}>Reset filters</button>
	</div>
</div>

<style>
	.filters {
		display: flex;
		flex-direction: column;
		gap: 18px;
	}

	.group {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	h4 {
		margin: 0;
		font-size: 12px;
		font-weight: 650;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-faint);
	}

	.seg {
		display: flex;
		flex-wrap: wrap;
		border-radius: var(--radius-sm);
		overflow: hidden;
		border: 1px solid var(--border-strong);
	}

	.seg button {
		flex: 1;
		min-width: 96px;
		padding: 7px 8px;
		font-size: 12.5px;
		font-weight: 550;
		color: var(--text-secondary);
		transition: background 0.12s ease, color 0.12s ease;
		border-right: 1px solid var(--border-strong);
	}

	.seg button:last-child {
		border-right: none;
	}

	.seg button.on {
		background: var(--accent-soft);
		color: var(--text);
	}

	.chips {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.add-row {
		display: flex;
		gap: 8px;
	}

	.add-row .type {
		width: auto;
		min-width: 110px;
	}

	.add-actions {
		display: flex;
		gap: 8px;
	}

	.range-row {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.range-row .input {
		width: 90px;
	}

	.foot {
		display: flex;
		justify-content: flex-end;
		border-top: 1px solid var(--border);
		padding-top: 12px;
	}
</style>