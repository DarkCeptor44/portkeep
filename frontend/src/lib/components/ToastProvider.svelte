<!-- 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { t } from "$lib/i18n/index.svelte";
	import { toast } from "$lib/toast.svelte";
</script>

<div
	class="fixed bottom-4 right-4 z-50 space-y-2 max-w-sm w-full pointer-events-none"
>
	{#each toast.items as item (item.id)}
		<div
			class="pointer-events-auto flex items-center justify-between p-3.5 rounded border shadow-xl text-sm animate-toast-snappy
                {item.type === 'success'
				? 'bg-slate-900/95 border-emerald-500/40 text-emerald-400 shadow-emerald-950/20'
				: 'bg-slate-900/95 border-rose-500/40 text-rose-400 shadow-rose-950/20'}"
		>
			<div class="flex items-center gap-2.5">
				{#if item.type === "success"}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="2"
						stroke="currentColor"
						class="w-4 h-4 text-emerald-500"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="2"
						stroke="currentColor"
						class="w-4 h-4 text-rose-500"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
				{/if}
				<span class="font-medium text-slate-200">{item.message}</span>
			</div>

			<button
				onclick={() => toast.dismiss(item.id)}
				title={t("common.dismiss")}
				class="cursor-pointer ml-4 p-0.5 text-slate-500 hover:text-slate-300 transition-colors"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="2"
					stroke="currentColor"
					class="w-3.5 h-3.5"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>
	{/each}
</div>
