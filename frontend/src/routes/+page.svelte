<!-- 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import type { Port, SortOption } from "$lib/types";
	import { appState } from "$lib/api.svelte";
	import { onMount } from "svelte";
	import { toast } from "$lib/toast.svelte";
	import { t } from "$lib/i18n/index.svelte";

	import DataList from "$lib/components/DataList.svelte";
	import PortForm from "$lib/components/PortForm.svelte";

	type PortField =
		| "port"
		| "description"
		| "is_listening"
		| "pid"
		| "process_name";

	const sortOptions = $derived<SortOption<PortField>[]>([
		{ id: "port", label: t("sort.port") },
		{ id: "description", label: t("sort.description") },
		{ id: "is_listening", label: t("sort.isListening") },
		{ id: "pid", label: t("sort.pid") },
		{ id: "process_name", label: t("sort.processName") },
	]);

	let deleteTarget = $state<Port | null>(null);
	let isDeleting = $state(false);
	let submitting = $state(false);
	let editingPort = $state<{
		port: number;
		description?: string | null;
	} | null>(null);

	onMount(() => {
		appState.fetchPorts();
	});

	function comparePorts(a: Port, b: Port, sortBy: PortField): number {
		switch (sortBy) {
			case "port":
				return a.port - b.port;
			case "description":
				return (a.description ?? "").localeCompare(b.description ?? "");
			case "is_listening":
				return (a.is_listening ? 1 : 0) - (b.is_listening ? 1 : 0);
			case "pid":
				return (a.pid ?? -1) - (b.pid ?? -1);
			case "process_name":
				return (a.process_name ?? "").localeCompare(
					b.process_name ?? "",
				);
			default:
				return 0;
		}
	}

	function handleAdd(item: Port) {
		editingPort = { port: item.port };
	}

	function handleEdit(item: Port) {
		editingPort = { port: item.port, description: item.description };
	}

	async function handleSave(payload: { port: number; description: string }) {
		submitting = true;

		const isEdit = !!editingPort?.description;

		try {
			if (isEdit) {
				await appState.editPort(payload.port, payload.description);
			} else {
				await appState.addPort(payload.port, payload.description);
			}

			toast.show(
				t(isEdit ? "success.editPort" : "success.addPort", {
					port: payload.port.toString(),
				}),
			);
			editingPort = null;
		} catch (err) {
			const message =
				err instanceof Error ? err.message : "An error occurred";
			console.error(message);
			toast.show(message, "error");
		} finally {
			submitting = false;
		}
	}

	function handleDelete(item: Port) {
		deleteTarget = item;
	}

	async function handleConfirmDelete() {
		if (!deleteTarget) return;
		isDeleting = true;

		try {
			await appState.deletePort(deleteTarget.port);

			toast.show(
				t("success.deletePort", { port: deleteTarget.port.toString() }),
			);
			deleteTarget = null;
		} catch (err) {
			const message =
				err instanceof Error ? err.message : "An error occurred";
			console.error(message);
			toast.show(message, "error");
		} finally {
			isDeleting = false;
		}
	}
</script>

<!-- creation form -->
<PortForm
	initialData={editingPort}
	{submitting}
	onSubmit={handleSave}
	onCancel={() => (editingPort = null)}
/>

<!-- ports list -->
<DataList
	items={appState.ports}
	loading={appState.loading}
	getItemKey={(item) => item.port}
	{sortOptions}
	defaultSortBy="port"
	defaultReverse={false}
	sortComparator={comparePorts}
	fuseOptions={{
		keys: ["port", "description", "is_listening", "pid", "process_name"],
		threshold: 0.3,
	}}
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

{#if deleteTarget}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm"
		onclick={() => !isDeleting && (deleteTarget = null)}
		role="presentation"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="rounded-xl border border-slate-800 bg-slate-900 p-6 max-w-sm w-full space-y-4 shadow-xl outline-none"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="space-y-2">
				<h3 class="text-base font-semibold text-slate-100">
					{t("common.deleteConfirmation")}
				</h3>
				<p class="text-sm text-slate-400">
					{t("common.deleteConfirmationText1")}
					<span class="font-mono font-medium text-indigo-400"
						>:{deleteTarget.port}</span
					>
					{#if deleteTarget.description}
						<span class="text-slate-300">
							({deleteTarget.description})</span
						>
					{/if}
					{t("common.deleteConfirmationText2")}
				</p>
			</div>

			<div class="flex items-center justify-end gap-3 pt-2">
				<button
					type="button"
					disabled={isDeleting}
					onclick={() => (deleteTarget = null)}
					class="rounded-lg border border-slate-700/60 bg-slate-800 px-3.5 py-2 text-xs font-medium text-slate-300 transition hover:bg-slate-700 focus:outline-none disabled:opacity-50 cursor-pointer"
				>
					{t("common.cancel")}
				</button>
				<button
					type="button"
					disabled={isDeleting}
					onclick={handleConfirmDelete}
					class="rounded-lg border border-red-900/50 bg-red-950/60 px-3.5 py-2 text-xs font-medium text-red-300 transition hover:bg-red-900/80 focus:outline-none disabled:opacity-50 cursor-pointer flex items-center gap-2"
				>
					{#if isDeleting}
						{t("common.deleting")}
					{:else}
						{t("common.delete")}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}
