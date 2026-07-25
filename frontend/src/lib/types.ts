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
