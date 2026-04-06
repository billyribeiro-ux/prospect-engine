import { browser } from "$app/environment";
import { DEFAULT_WS_CONNECT_TIMEOUT_MS } from "$lib/constants/api";
import { logError, logInfo } from "$lib/utils/logger";

export type WebsocketConnectionStatus =
	| "idle"
	| "connecting"
	| "open"
	| "closing"
	| "closed"
	| "error";

class WebsocketConnectionState {
	status = $state<WebsocketConnectionStatus>("idle");
	lastError = $state<string | undefined>(undefined);

	private socket: WebSocket | undefined;
	private connectTimeoutId: ReturnType<typeof setTimeout> | undefined;

	get isConnected(): boolean {
		return this.status === "open";
	}

	connect(url: string, protocols?: string | string[]): void {
		if (!browser) {
			return;
		}
		this.disconnect();
		this.status = "connecting";
		this.lastError = undefined;

		this.connectTimeoutId = setTimeout(() => {
			if (this.status === "connecting") {
				this.lastError = "WebSocket connection timed out";
				this.status = "error";
				this.socket?.close();
				this.socket = undefined;
			}
		}, DEFAULT_WS_CONNECT_TIMEOUT_MS);

		const ws = new WebSocket(url, protocols);
		this.socket = ws;

		ws.addEventListener("open", () => {
			if (this.connectTimeoutId !== undefined) {
				clearTimeout(this.connectTimeoutId);
				this.connectTimeoutId = undefined;
			}
			this.status = "open";
			logInfo("websocket", "socket open", { url });
		});

		ws.addEventListener("close", () => {
			if (this.connectTimeoutId !== undefined) {
				clearTimeout(this.connectTimeoutId);
				this.connectTimeoutId = undefined;
			}
			this.status = "closed";
			this.socket = undefined;
		});

		ws.addEventListener("error", () => {
			if (this.connectTimeoutId !== undefined) {
				clearTimeout(this.connectTimeoutId);
				this.connectTimeoutId = undefined;
			}
			this.lastError = "WebSocket connection error";
			this.status = "error";
			logError("websocket", "socket error", { url });
		});
	}

	disconnect(code?: number, reason?: string): void {
		if (this.connectTimeoutId !== undefined) {
			clearTimeout(this.connectTimeoutId);
			this.connectTimeoutId = undefined;
		}
		const current = this.socket;
		if (!current) {
			this.status = "idle";
			return;
		}
		this.status = "closing";
		current.close(code, reason);
	}
}

export const websocketState = new WebsocketConnectionState();
