<script lang="ts">
	import type { Port } from "$lib/types";
	import { appState } from "$lib/api.svelte";
	import { onMount } from "svelte";
	import { t } from "$lib/i18n/index.svelte";

	import DataList from "$lib/components/DataList.svelte";

	let port = $state("");
	let description = $state("");

	onMount(() => {
		appState.fetchPorts();
	});

	function handleSubmit(e: Event) {
		e.preventDefault();
	}

	function handleAdd(item: Port) {
		return;
	}

	function handleDelete(item: Port) {
		return;
	}

	function handleEdit(item: Port) {
		return;
	}
</script>

<!-- creation form -->
<section class="mb-8">
	<form
		onsubmit={handleSubmit}
		class="rounded-xl border border-slate-800 bg-slate-800/40 p-4 shadow-sm backdrop-blur"
	>
		<div
			class="mb-3 text-xs font-medium uppercase tracking-wider text-slate-400"
		>
			{t("form.title")}
		</div>
		<div class="flex flex-col gap-3 sm:flex-row">
			<div class="w-full sm:w-36">
				<input
					type="number"
					bind:value={port}
					placeholder={t("form.portPlaceholder")}
					min="1"
					max="65535"
					class="w-full rounded-lg border border-slate-700 bg-slate-900 px-3.5 py-2 text-sm text-slate-100 placeholder-slate-500 transition focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
					required
				/>
			</div>

			<div class="flex-1">
				<input
					type="text"
					bind:value={description}
					placeholder={t("form.descPlaceholder")}
					class="w-full rounded-lg border border-slate-700 bg-slate-900 px-3.5 py-2 text-sm text-slate-100 placeholder-slate-500 transition focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
					required
				/>
			</div>

			<button
				type="submit"
				class="inline-flex items-center justify-center rounded-lg bg-indigo-600 px-5 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 active:bg-indigo-700 cursor-pointer"
			>
				{t("form.buttonAdd")}
			</button>
		</div>
	</form>
</section>

<!-- ports list -->
<DataList
	items={appState.ports}
	loading={appState.loading}
	getItemKey={(item) => item.port}
>
	{#snippet children(item, {})}
		<div class="flex items-center gap-4">
			<!-- port number -->
			<div
				class="w-18 font-mono text-xl font-bold {item.is_listening
					? 'text-indigo-400'
					: 'text-slate-500'}"
			>
				:{item.port}
			</div>

			<div>
				<!-- description -->
				<div
					class="text-base font-medium {item.description
						? 'text-slate-100'
						: 'text-slate-500'}"
				>
					{item.description || t("list.noDesc")}
				</div>

				<!-- status -->
				<div class="flex items-center gap-2 text-xs text-slate-400">
					<!-- active -->
					{#if item.is_listening}
						<span
							class="inline-flex items-center gap-1.5 text-emerald-400 font-medium"
						>
							<span
								class="h-1.5 w-1.5 rounded-full bg-emerald-400"
							></span>
							{t("list.listening")}
						</span>
					{:else}
						<span
							class="inline-flex items-center gap-1.5 text-slate-500 font-medium"
						>
							<span class="h-1.5 w-1.5 rounded-full bg-slate-600"
							></span>
							{t("list.inactive")}
						</span>
					{/if}

					<!-- process info -->
					{#if item.pid || item.process_name}
						<span class="text-slate-700 select-none">•</span>
						<div class="flex items-center gap-1.5 font-mono">
							{#if item.process_name}
								<span
									class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] font-medium text-slate-300 border border-slate-700/50"
								>
									{item.process_name}
								</span>
							{/if}
							{#if item.pid}
								<span
									class="rounded bg-slate-800/60 px-1.5 py-0.5 text-[11px] text-slate-400 border border-slate-700/30"
									>PID {item.pid}</span
								>
							{/if}
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- actions -->
		<div class="flex items-center gap-2">
			{#if item.description}
				<button
					type="button"
					onclick={() => handleEdit(item)}
					class="rounded-md border border-slate-700/60 bg-slate-800 px-3 py-1.5 text-xs font-medium text-slate-300 transition hover:bg-slate-700 focus:outline-none cursor-pointer select-none"
				>
					{t("list.edit")}
				</button>

				<button
					type="button"
					onclick={() => handleDelete(item)}
					class="rounded-md border border-red-900/40 bg-red-950/30 px-3 py-1.5 text-xs font-medium text-red-400 transition hover:bg-red-900/40 focus:outline-none cursor-pointer select-none"
				>
					{t("list.delete")}
				</button>
			{:else}
				<button
					type="button"
					onclick={() => handleAdd(item)}
					class="rounded-md border border-indigo-500/40 bg-indigo-950/40 px-3 py-1.5 text-xs font-medium text-indigo-300 transition hover:bg-indigo-900/50 focus:outline-none cursor-pointer select-none"
				>
					{t("list.add")}
				</button>
			{/if}
		</div>
	{/snippet}
</DataList>
