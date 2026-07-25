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
			searchPlaceholder: "Search ports or descriptions...",
			title: "Registered Ports",
		},
		stats: {
			active: "Active",
			allocated: "Allocated",
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
			searchPlaceholder: "Pesquisar portas ou descrições...",
			title: "Portas Registradas",
		},
		stats: {
			active: "Ativo",
			allocated: "Alocado",
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
