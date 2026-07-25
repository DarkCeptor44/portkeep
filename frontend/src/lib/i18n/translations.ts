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
			delete: "Delete",
			edit: "Edit",
			empty: "No ports registered",
			inactive: "Inactive",
			listening: "Listening",
			loading: "Loading...",
			noDesc: "Unregistered Port",
			searchPlaceholder: "Search ports or descriptions...",
			title: "Registered Ports",
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
			delete: "Deletar",
			edit: "Editar",
			empty: "Nenhuma porta registrada",
			inactive: "Inativo",
			listening: "Ouvindo",
			loading: "Carregando...",
			noDesc: "Porta Não Registrada",
			searchPlaceholder: "Pesquisar portas ou descrições...",
			title: "Portas Registradas",
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
