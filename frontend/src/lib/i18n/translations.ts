// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

export const translations = {
	en: {
		meta: { label: "English" },
		common: {
			title: "PortKeep",
		},
		form: {
			buttonAdd: "Add Port",
			descPlaceholder: "Description (e.g. Web API Server)",
			portPlaceholder: "Port (8080)",
			title: "Register New Port",
		},
		list: {
			add: "Add",
			clearSearch: "Clear search",
			delete: "Delete",
			edit: "Edit",
			empty: "No ports registered",
			inactive: "Inactive",
			listening: "Listening",
			loading: "Loading...",
			noDesc: "Unregistered Port",
			noSearchResults: "No results found for",
			searchPlaceholder: "Search ports or descriptions...",
			title: "Registered Ports",
		},
		sort: {
			description: "Description",
			isListening: "Listening",
			pid: "PID",
			port: "Port",
			processName: "Process Name",
			sortBy: "Sort By",
		},
		stats: {
			active: "Active",
			allocated: "Registered",
		},
	},

	"pt-BR": {
		meta: { label: "Português" },
		common: {
			title: "PortKeep",
		},
		form: {
			buttonAdd: "Adicionar Porta",
			descPlaceholder: "Descrição (e.g. Servidor Web API)",
			portPlaceholder: "Porta (8080)",
			title: "Registrar Nova Porta",
		},
		list: {
			add: "Adicionar",
			clearSearch: "Limpar pesquisa",
			delete: "Deletar",
			edit: "Editar",
			empty: "Nenhuma porta registrada",
			inactive: "Inativo",
			listening: "Ouvindo",
			loading: "Carregando...",
			noDesc: "Porta Não Registrada",
			noSearchResults: "Nenhum resultado encontrado para",
			searchPlaceholder: "Pesquisar portas ou descrições...",
			title: "Portas Registradas",
		},
		sort: {
			description: "Descrição",
			isListening: "Ouvindo",
			pid: "PID",
			port: "Porta",
			processName: "Nome do Processo",
			sortBy: "Ordenar Por",
		},
		stats: {
			active: "Ativo",
			allocated: "Registrado",
		},
	},
} as const;

type TranslatableStructure = Omit<typeof translations.en, "meta">;

type NestedKeys<T> = T extends object
	? {
			[K in keyof T & string]: T[K] extends object
				? `${K}.${NestedKeys<T[K]>}`
				: K;
		}[keyof T & string]
	: never;

export type TranslationKey = NestedKeys<TranslatableStructure> | (string & {});
