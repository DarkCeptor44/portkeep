// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

export const translations = {
	en: {
		meta: { label: "English" },
		common: {
			adding: "Adding...",
			cancel: "Cancel",
			delete: "Delete",
			deleting: "Deleting...",
			deleteConfirmation: "Confirm deletion",
			deleteConfirmationText1: "Are you sure you want to delete",
			deleteConfirmationText2: "? This action cannot be undone.",
			dismiss: "Dismiss",
			save: "Save Changes",
			saving: "Saving...",
			title: "PortKeep",
		},
		error: {
			addPort: "Failed to add port",
			deletePort: "Failed to delete port",
			editPort: "Failed to edit port",
		},
		form: {
			buttonAdd: "Add Port",
			descPlaceholder: "Description (e.g. Web API Server)",
			portPlaceholder: "Port (8080)",
			titleAdd: "Register New Port",
			titleEdit: "Edit Port",
		},
		list: {
			add: "Add",
			clearSearch: "Clear search",
			delete: "Delete",
			edit: "Edit",
			empty: "No ports registered",
			filterAll: "All",
			filterRegistered: "Registered",
			filterUnregistered: "Unregistered",
			inactive: "Inactive",
			listening: "Listening",
			loading: "Loading...",
			noDesc: "Unregistered Port",
			noSearchResults: "No results found for",
			searchPlaceholder: "Search ports or descriptions...",
			title: "Ports",
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
		success: {
			addPort: "Port {port} was added successfully",
			deletePort: "Port {port} deleted successfully",
			editPort: "Port {port} was updated successfully",
		},
	},

	"pt-BR": {
		meta: { label: "Português" },
		common: {
			adding: "Adicionando...",
			cancel: "Cancelar",
			delete: "Excluir",
			deleting: "Excluindo...",
			deleteConfirmation: "Confirmar exclusão",
			deleteConfirmationText1: "Tem certeza que deseja excluir",
			deleteConfirmationText2: "? Esta ação não pode ser desfeita.",
			dismiss: "Descartar",
			save: "Salvar Alterações",
			saving: "Salvando...",
			title: "PortKeep",
		},
		error: {
			addPort: "Falha ao adicionar porta",
			deletePort: "Falha ao excluir porta",
			editPort: "Falha ao editar porta",
		},
		form: {
			buttonAdd: "Adicionar Porta",
			descPlaceholder: "Descrição (e.g. Servidor Web API)",
			portPlaceholder: "Porta (8080)",
			titleAdd: "Registrar Nova Porta",
			titleEdit: "Editar Porta",
		},
		list: {
			add: "Adicionar",
			clearSearch: "Limpar pesquisa",
			delete: "Deletar",
			edit: "Editar",
			empty: "Nenhuma porta registrada",
			filterAll: "Todos",
			filterRegistered: "Registradas",
			filterUnregistered: "Não Registradas",
			inactive: "Inativo",
			listening: "Ouvindo",
			loading: "Carregando...",
			noDesc: "Porta Não Registrada",
			noSearchResults: "Nenhum resultado encontrado para",
			searchPlaceholder: "Pesquisar portas ou descrições...",
			title: "Portas",
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
		success: {
			addPort: "Porta {port} adicionada com sucesso",
			deletePort: "Porta {port} excluída com sucesso",
			editPort: "Porta {port} atualizada com sucesso",
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
