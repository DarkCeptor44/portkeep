// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/**
 * Represents a port received from the backend
 */
export interface Port {
	/**
	 * The port number
	 */
	port: number;

	/**
	 * The description of the port
	 */
	description: string | null;

	/**
	 * Whether the port is currently listening
	 */
	is_listening: boolean;

	/**
	 * The PID of the process listening to the port
	 */
	pid: number | null;

	/**
	 * The name of the process listening to the port
	 */
	process_name: string | null;
}

/**
 * Represents a sort option
 */
export interface SortOption<T> {
	/**
	 * The field to sort by
	 */
	id: T;

	/**
	 * The field label
	 */
	label: string;
}
