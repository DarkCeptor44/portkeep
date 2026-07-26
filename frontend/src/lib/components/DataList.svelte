<script lang="ts" generics="TData,TSort extends string=string">
	import type { Snippet } from "svelte";
	import { t } from "$lib/i18n/index.svelte";
	import { slide } from "svelte/transition";

	interface Props<TData, TSort extends string = string> {
		items: TData[];
		loading: boolean;

		children: Snippet<[TData, {}]>;
		getItemKey: (item: TData) => string | number;
	}

	let { items, loading, children, getItemKey }: Props<TData, TSort> =
		$props();

	let searchInputEl = $state<HTMLInputElement | null>(null);

	function handleKeydown(e: KeyboardEvent) {
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

		<!-- search -->
		<div class="relative w-full sm:w-64">
			<input
				type="text"
				placeholder={t("list.searchPlaceholder")}
				class="w-full rounded-lg border border-slate-800 bg-slate-900 py-1.5 pl-3 pr-8 text-xs text-slate-200 placeholder-slate-500 focus:border-slate-700 focus:outline-none"
			/>
		</div>
	</div>

	<div
		class="overflow-hidden rounded-xl border border-slate-800 bg-slate-900/60 shadow-sm"
	>
		{#if loading}
			<div class="p-6 text-center text-slate-400 text-sm">
				{t("list.loading")}
			</div>
		{:else if items.length === 0}
			<div class="p-6 text-center text-slate-400 text-sm">
				{t("list.empty")}
			</div>
		{:else}
			<ul class="divide-y divide-slate-800/60">
				{#each items as item (getItemKey(item))}
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
