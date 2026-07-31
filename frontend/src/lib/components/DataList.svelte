<script lang="ts" generics="TData, TSort extends string=string">
	import type { Snippet } from "svelte";
	import type { SortOption } from "$lib/types";
	import Fuse, { type IFuseOptions } from "fuse.js";
	import { t } from "$lib/i18n/index.svelte";
	import { slide } from "svelte/transition";

	import SortControls from "./SortControls.svelte";

	type FilterMode = "all" | "registered" | "unregistered";

	interface Props<TData, TSort extends string = string> {
		items: TData[];
		loading: boolean;

		// sort options
		sortOptions?: SortOption<TSort>[];
		defaultSortBy?: TSort;
		defaultReverse?: boolean;
		sortComparator?: (a: TData, b: TData, sortBy: TSort) => number;

		// search options
		fuseOptions?: IFuseOptions<TData>;

		// filter options
		filterPredicate?: (item: TData, mode: FilterMode) => boolean;

		children: Snippet<[TData, {}]>;
		getItemKey: (item: TData) => string | number;
	}

	let {
		items,
		loading,
		sortOptions,
		defaultSortBy,
		defaultReverse = false,
		sortComparator,
		fuseOptions,
		filterPredicate,
		children,
		getItemKey,
	}: Props<TData, TSort> = $props();

	let searchQuery = $state("");
	let debouncedQuery = $state("");
	let activeSortBy = $state<TSort | undefined>(undefined);
	let reverse = $state(false);
	let activeFilter = $state<FilterMode>("registered");
	let searchInputEl = $state<HTMLInputElement | null>(null);

	$effect(() => {
		activeSortBy = defaultSortBy ?? sortOptions?.[0]?.id;
		reverse = defaultReverse;

		const query = searchQuery;
		const timer = setTimeout(() => {
			debouncedQuery = query;
		}, 200);
		return () => clearTimeout(timer);
	});

	let baseFilteredItems = $derived.by(() => {
		if (!filterPredicate || activeFilter === "all") return items;
		return items.filter((item) => filterPredicate(item, activeFilter));
	});
	let fuse = $derived(
		fuseOptions ? new Fuse(baseFilteredItems, fuseOptions) : null,
	);
	let searchFilteredItems = $derived.by(() => {
		const query = debouncedQuery.trim();
		if (!fuse || !query) return baseFilteredItems;
		return fuse.search(query).map((res) => res.item);
	});
	let processedItems = $derived.by(() => {
		if (!sortOptions || !activeSortBy || !sortComparator) {
			return searchFilteredItems;
		}

		const currentSort = activeSortBy;
		const list = [...searchFilteredItems];
		list.sort((a, b) => sortComparator(a, b, currentSort));
		if (reverse) list.reverse();
		return list;
	});
	let isSearchEmpty = $derived(
		Boolean(
			fuseOptions && debouncedQuery.trim() && processedItems.length === 0,
		),
	);

	function toggleSort(field: TSort) {
		if (activeSortBy === field) {
			reverse = !reverse;
		} else {
			activeSortBy = field;
			reverse = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!fuseOptions) return;

		const isFocused = document.activeElement === searchInputEl;
		if (e.key === "Escape" && isFocused) {
			e.preventDefault();
			searchInputEl?.blur();
			return;
		}

		if (e.key === "/") {
			const target = e.target as HTMLElement | null;
			if (
				target &&
				(target.tagName === "INPUT" ||
					target.tagName === "TEXTAREA" ||
					target.isContentEditable)
			) {
				return;
			}

			e.preventDefault();
			searchInputEl?.focus();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<section>
	<div
		class="mb-4 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between"
	>
		<h2 class="text-lg font-semibold text-slate-200">{t("list.title")}</h2>

		<div class="flex flex-wrap items-center gap-3">
			<!-- filters -->
			{#if filterPredicate}
				<div
					class="inline-flex rounded-lg border border-slate-800 bg-slate-900/80 p-0.5 text-xs font-medium"
				>
					<button
						type="button"
						onclick={() => (activeFilter = "all")}
						class="rounded-md px-2.5 py-1 transition cursor-pointer select-none {activeFilter ===
						'all'
							? 'bg-slate-800 text-slate-100 shadow-sm'
							: 'text-slate-400 hover:text-slate-200'}"
					>
						{t("list.filterAll")}
					</button>
					<button
						type="button"
						onclick={() => (activeFilter = "registered")}
						class="rounded-md px-2.5 py-1 transition cursor-pointer select-none {activeFilter ===
						'registered'
							? 'bg-slate-800 text-slate-100 shadow-sm'
							: 'text-slate-400 hover:text-slate-200'}"
					>
						{t("list.filterRegistered")}
					</button>
					<button
						type="button"
						onclick={() => (activeFilter = "unregistered")}
						class="rounded-md px-2.5 py-1 transition cursor-pointer select-none {activeFilter ===
						'unregistered'
							? 'bg-slate-800 text-slate-100 shadow-sm'
							: 'text-slate-400 hover:text-slate-200'}"
					>
						{t("list.filterUnregistered")}
					</button>
				</div>
			{/if}

			<!-- search -->
			{#if fuseOptions}
				<div class="relative w-full sm:w-64">
					<input
						type="text"
						placeholder={t("list.searchPlaceholder")}
						bind:value={searchQuery}
						bind:this={searchInputEl}
						class="w-full rounded-lg border border-slate-800 bg-slate-900 py-1.5 pl-3 pr-8 text-xs text-slate-200 placeholder-slate-500 focus:border-slate-700 focus:outline-none transition"
					/>
					{#if searchQuery}
						<button
							onclick={() => (searchQuery = "")}
							class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-500 hover:text-slate-300 text-xs px-1 cursor-pointer"
						>
							✕
						</button>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	{#if sortOptions && activeSortBy && sortComparator}
		<SortControls
			options={sortOptions}
			sortBy={activeSortBy}
			{reverse}
			onToggle={toggleSort}
		/>
	{/if}

	<div
		class="overflow-hidden rounded-xl border border-slate-800 bg-slate-900/60 shadow-sm"
	>
		{#if loading}
			<div class="p-6 text-center text-slate-400 text-sm">
				{t("list.loading")}
			</div>
		{:else if isSearchEmpty}
			<div class="p-6 text-center text-slate-400 text-sm space-y-1">
				<p>
					{t("list.noSearchResults")}
					<span class="font-bold">{debouncedQuery}</span>
				</p>
				<button
					onclick={() => (searchQuery = "")}
					class="text-indigo-400 hover:text-indigo-300 text-xs hover:underline cursor-pointer"
				>
					{t("list.clearSearch")}
				</button>
			</div>
		{:else if processedItems.length === 0}
			<div class="p-6 text-center text-slate-400 text-sm">
				{t("list.empty")}
			</div>
		{:else}
			<ul class="divide-y divide-slate-800/60">
				{#each processedItems as item (getItemKey(item))}
					<li
						class="flex items-center justify-between p-4 transition hover:bg-slate-800/30"
						transition:slide={{ duration: 150 }}
					>
						{@render children(item, {})}
					</li>
				{/each}
			</ul>
		{/if}
	</div>
</section>
