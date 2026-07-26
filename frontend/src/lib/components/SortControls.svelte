<script lang="ts" generics="T extends string">
	import type { SortOption } from "$lib/types";
	import { t } from "$lib/i18n/index.svelte";

	interface Props {
		options: SortOption<T>[];
		sortBy: T;
		reverse: boolean;
		onToggle: (field: T) => void;
	}

	let { options, sortBy, reverse, onToggle }: Props = $props();
</script>

<div
	class="flex flex-wrap items-center gap-2 p-3 rounded-xl border border-slate-800/80 bg-slate-900/40 text-sm mb-3"
>
	<span class="font-medium text-slate-400 whitespace-nowrap px-1"
		>{t("sort.sortBy")}:</span
	>

	<div class="flex flex-wrap items-center gap-1.5">
		{#each options as option (option.id)}
			{@const isActive = sortBy === option.id}
			<button
				type="button"
				onclick={() => onToggle(option.id)}
				class="inline-flex cursor-pointer items-center gap-1.5 rounded-lg border px-3 py-1.5 text-xs font-medium transition select-none focus:outline-none {isActive
					? 'border-indigo-500/50 bg-indigo-950/60 text-indigo-300 shadow-sm shadow-indigo-950/50'
					: 'border-slate-800 bg-slate-800/40 text-slate-400 hover:border-slate-700 hover:bg-slate-800 hover:text-slate-200'}"
			>
				<span>{option.label}</span>
				{#if isActive}
					<span class="text-[12px] text-indigo-400 font-mono">
						{reverse ? "↓" : "↑"}
					</span>
				{/if}
			</button>
		{/each}
	</div>
</div>
