<script lang="ts">
	import { i18n, type Language } from "$lib/i18n/index.svelte";

	let isOpen = $state(false);

	const languages: Array<{ code: Language; label: string }> = [
		{ code: "en", label: "English" },
		{ code: "pt-BR", label: "Português" },
	];

	function selectLanguage(code: Language) {
		i18n.setLanguage(code);
		isOpen = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function handleBlur(event: FocusEvent) {
		const currentTarget = event.currentTarget as HTMLElement;
		if (!currentTarget.contains(event.relatedTarget as Node)) {
			isOpen = false;
		}
	}
</script>

<div class="relative inline-block text-left" onfocusout={handleBlur}>
	<!-- trigger -->
	<button
		type="button"
		onclick={toggleDropdown}
		class="inline-flex items-center gap-2 rounded-lg border border-slate-800 bg-slate-900 px-2 py-1.5 text-xs font-medium text-slate-300 transition hover:border-slate-700 hover:bg-slate-800 hover:text-slate-100 focus:outline-none focus:ring-1 focus:ring-indigo-500 cursor-pointer select-none"
		aria-expanded={isOpen}
		aria-haspopup="true"
	>
		<span class="text-sm">🌐</span>
		<span class="uppercase tracking-wider">
			{i18n.currentLanguage}
		</span>

		<svg
			class="h-3.5 w-3.5 text-slate-400 transition-transform duration-150 {isOpen
				? 'rotate-180'
				: ''}"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 20 20"
			fill="currentColor"
		>
			<path
				fill-rule="evenodd"
				d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
				clip-rule="evenodd"
			/>
		</svg>
	</button>

	<!-- dropdown -->
	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-2 w-36 origin-top-right rounded-lg border border-slate-700/80 bg-slate-800 py-1 shadow-xl shadow-black/40 ring-1 ring-black/5 focus:outline-none select-none"
			role="menu"
		>
			{#each i18n.availableLanguages as lang}
				<button
					type="button"
					onclick={() => selectLanguage(lang.code)}
					class="cursor-pointer flex w-full items-center justify-between px-3 py-2 text-left text-sm text-slate-300 transition hover:bg-slate-700/60 hover:text-slate-100 {i18n.currentLanguage ===
					lang.code
						? 'font-semibold text-indigo-400'
						: ''}"
					role="menuitem"
				>
					<span>{lang.label}</span>
					{#if i18n.currentLanguage === lang.code}
						<span class="h-1.5 w-1.5 rounded-full bg-indigo-400"
						></span>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>
