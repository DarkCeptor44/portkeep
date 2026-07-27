// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

import type { Port } from "$lib/types";

class AppState {
	ports = $state<Port[]>([]);
	loading = $state(false);

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
