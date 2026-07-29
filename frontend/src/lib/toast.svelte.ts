// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

export interface Toast {
	id: string;
	message: string;
	type: "success" | "error";
}

class ToastManager {
	#toasts = $state<Toast[]>([]);

	get items() {
		return this.#toasts;
	}

	show(
		message: string,
		type: "success" | "error" = "success",
		durationMs = 4000,
	) {
		const id = generateId();
		this.#toasts = [...this.#toasts, { id, message, type }];

		setTimeout(() => {
			this.dismiss(id);
		}, durationMs);
	}

	dismiss(id: string) {
		this.#toasts = this.#toasts.filter((t) => t.id !== id);
	}
}

function generateId(): string {
	if (
		typeof crypto !== "undefined" &&
		typeof crypto.randomUUID === "function"
	) {
		return crypto.randomUUID();
	}
	return Date.now().toString(36) + Math.random().toString(36).substring(2, 9);
}

export const toast = new ToastManager();
