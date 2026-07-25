<script lang="ts">
	import { t } from "$lib/i18n/index.svelte";

	let port = $state("");
	let description = $state("");

	function handleSubmit(e: Event) {
		e.preventDefault();
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

	<!-- TODO ports table -->
</section>
