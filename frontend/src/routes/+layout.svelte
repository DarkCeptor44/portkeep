<!-- 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import "./layout.css";
	import { onMount } from "svelte";
	import { appState } from "$lib/api.svelte";
	import { t } from "$lib/i18n/index.svelte";

	import LangPicker from "$lib/components/LangPicker.svelte";

	onMount(() => {
		if (import.meta.env.DEV) {
			import("eruda").then((eruda) => eruda.default.init());
		}
	});

	const allocatedCount = $derived(
		appState.ports.filter((item) => item.description !== null).length,
	);
	const activeCount = $derived(
		appState.ports.filter((item) => item.is_listening).length,
	);

	let { children } = $props();
</script>

<svelte:head>
	<title>{t("common.title")}</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-slate-900 text-slate-100">
	<!-- navbar -->
	<header
		class="sticky top-0 z-50 border-b border-slate-800 bg-slate-900/90 backdrop-blur"
	>
		<div
			class="mx-auto flex h-13 max-w-6xl items-center justify-between px-6"
		>
			<!-- logo -->
			<div class="flex items-center gap-3 select-none">
				<div
					class="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-600 font-mono text-sm font-bold text-white shadow-sm"
				>
					pk
				</div>
				<span class="text-lg font-semibold tracking-wide text-slate-100"
					>{t("common.title")}</span
				>
			</div>

			<div class="flex items-center gap-4">
				<!-- stats -->
				<div
					class="hidden sm:flex items-center gap-3 text-xs font-medium"
				>
					<span
						class="rounded-md border border-transparent bg-slate-800 px-2.5 py-1 text-slate-400"
					>
						{t("stats.allocated")}:
						<strong class="text-slate-200">{allocatedCount}</strong>
					</span>
					<span
						class="rounded-md border border-emerald-800/40 bg-emerald-950/60 px-2.5 py-1 text-emerald-400"
					>
						{t("stats.active")}:
						<strong class="text-emerald-300">{activeCount}</strong>
					</span>
				</div>

				<!-- language picker -->
				<LangPicker />
			</div>
		</div>
	</header>

	<!-- main -->
	<main class="mx-auto w-full max-w-6xl flex-1 px-6 py-8">
		{@render children()}
	</main>
</div>
