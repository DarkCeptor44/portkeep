// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

import type { Port } from "$lib/types";
import { t } from "./i18n/index.svelte";

class AppState {
	ports = $state<Port[]>([]);
	loading = $state(false);

	async addPort(port: number, description: string) {
		const res = await fetch("/api/v1/port", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				port,
				description,
			}),
		});

		if (!res.ok) {
			const message = await res.text();
			throw new Error(message || t("error.addPort"));
		}

		const existingIndex = this.ports.findIndex((p) => p.port === port);
		if (existingIndex !== -1) {
			this.ports[existingIndex].description = description;
		} else {
			this.ports.push({
				port,
				description,
				is_listening: false,
				pid: null,
				process_name: null,
			});
		}
	}

	async deletePort(port: number) {
		const res = await fetch(`/api/v1/port/${port}`, {
			method: "DELETE",
		});

		if (!res.ok) {
			const message = await res.text();
			throw new Error(message || t("error.deletePort"));
		}

		this.ports = this.ports.filter((p) => p.port !== port);
	}

	async editPort(port: number, description: string) {
		const res = await fetch("/api/v1/port", {
			method: "PUT",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				port,
				description,
			}),
		});

		if (!res.ok) {
			const message = await res.text();
			throw new Error(message || t("error.editPort"));
		}

		const item = this.ports.find((p) => p.port === port);
		if (item) {
			item.description = description;
		}
	}

	async fetchPorts() {
		this.loading = true;
		try {
			const res = await fetch("/api/v1/ports", { cache: "no-store" });

			if (res.ok) {
				const data = await res.json();
				this.ports = data.map((p: Port) => ({
					...p,
				}));
			}
		} catch (err) {
			console.error("Failed to fetch ports", err);
		} finally {
			this.loading = false;
		}
	}
}

export const appState = new AppState();
