<!-- 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { t } from "$lib/i18n/index.svelte";

	type PortPayload = {
		port: number;
		description: string;
	};

	interface Props {
		initialData?: { port: number; description?: string | null } | null;
		submitting?: boolean;
		onSubmit: (payload: PortPayload) => void;
		onCancel?: () => void;
	}

	let {
		initialData = null,
		submitting = false,
		onSubmit,
		onCancel,
	}: Props = $props();

	let port = $state<number>(0);
	let description = $state("");
	let descInput = $state<HTMLInputElement | null>(null);

	const isEdit = $derived(!!initialData?.description);

	$effect(() => {
		port = initialData?.port ?? 0;
		description = initialData?.description ?? "";

		if (initialData) {
			descInput?.scrollIntoView({ behavior: "smooth", block: "center" });
			descInput?.focus();
			descInput?.select();
		}
	});

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (port === 0 || !description.trim() || submitting) return;

		onSubmit({
			port,
			description: description.trim(),
		});

		if (!initialData) {
			port = 0;
			description = "";
		}
	}
</script>

<section class="mb-8">
	<form
		onsubmit={handleSubmit}
		class="rounded-xl border border-slate-800 bg-slate-800/40 p-4 shadow-sm backdrop-blur"
	>
		<div
			class="mb-3 text-xs font-medium uppercase tracking-wider text-slate-400"
		>
			{isEdit ? t("form.titleEdit") : t("form.titleAdd")}
		</div>

		<div class="flex flex-col gap-3 sm:flex-row">
			<div class="w-full sm:w-36">
				<input
					type="number"
					id="port-number"
					bind:value={port}
					disabled={!!initialData}
					placeholder={t("form.portPlaceholder")}
					min="1"
					max="65535"
					class="w-full rounded-lg border border-slate-700 bg-slate-900 px-3.5 py-2 text-sm text-slate-100 placeholder-slate-500 transition focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
				/>
			</div>

			<div class="flex-1">
				<input
					bind:this={descInput}
					type="text"
					id="port-desc"
					bind:value={description}
					placeholder={t("form.descPlaceholder")}
					class="w-full rounded-lg border border-slate-700 bg-slate-900 px-3.5 py-2 text-sm text-slate-100 placeholder-slate-500 transition focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
				/>
			</div>

			<div class="flex items-center gap-2">
				{#if initialData && onCancel}
					<button
						type="button"
						onclick={onCancel}
						class="px-3 py-1.5 text-xs text-slate-400 hover:text-slate-200 rounded transition-colors cursor-pointer"
					>
						{t("common.cancel")}
					</button>
				{/if}

				<button
					type="submit"
					disabled={submitting || port === 0 || !description.trim()}
					class="inline-flex items-center justify-center rounded-lg bg-indigo-600 px-5 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 active:bg-indigo-700 disabled:text-slate-600 disabled:bg-slate-700 {submitting ||
					port === 0 ||
					!description.trim()
						? ''
						: 'cursor-pointer'}"
				>
					{submitting
						? isEdit
							? t("common.saving")
							: t("common.adding")
						: isEdit
							? t("common.save")
							: t("form.buttonAdd")}
				</button>
			</div>
		</div>
	</form>
</section>
