export type LogLevel = "debug" | "info" | "warn" | "error";

const LOG_LEVEL_ORDER: Record<LogLevel, number> = {
	debug: 10,
	info: 20,
	warn: 30,
	error: 40,
};

const MIN_LEVEL: LogLevel = import.meta.env.DEV ? "debug" : "info";

export interface LogRecord {
	readonly timestampIso: string;
	readonly level: LogLevel;
	readonly context: string;
	readonly message: string;
	readonly meta: Readonly<Record<string, unknown>> | undefined;
}

function shouldEmit(level: LogLevel): boolean {
	return LOG_LEVEL_ORDER[level] >= LOG_LEVEL_ORDER[MIN_LEVEL];
}

function serialize(record: LogRecord): string {
	return JSON.stringify(record);
}

function emitStructured(record: LogRecord): void {
	const sink = globalThis.console;
	if (record.level === "error") {
		sink.error(serialize(record));
		return;
	}
	if (!import.meta.env.DEV) {
		return;
	}
	if (record.level === "warn") {
		sink.warn(serialize(record));
		return;
	}
	sink.info(serialize(record));
}

export function logDebug(
	context: string,
	message: string,
	meta?: Readonly<Record<string, unknown>>,
): void {
	if (!shouldEmit("debug")) return;
	const record: LogRecord = {
		timestampIso: new Date().toISOString(),
		level: "debug",
		context,
		message,
		meta,
	};
	emitStructured(record);
}

export function logInfo(
	context: string,
	message: string,
	meta?: Readonly<Record<string, unknown>>,
): void {
	if (!shouldEmit("info")) return;
	const record: LogRecord = {
		timestampIso: new Date().toISOString(),
		level: "info",
		context,
		message,
		meta,
	};
	emitStructured(record);
}

export function logWarn(
	context: string,
	message: string,
	meta?: Readonly<Record<string, unknown>>,
): void {
	if (!shouldEmit("warn")) return;
	const record: LogRecord = {
		timestampIso: new Date().toISOString(),
		level: "warn",
		context,
		message,
		meta,
	};
	emitStructured(record);
}

export function logError(
	context: string,
	message: string,
	meta?: Readonly<Record<string, unknown>>,
): void {
	if (!shouldEmit("error")) return;
	const record: LogRecord = {
		timestampIso: new Date().toISOString(),
		level: "error",
		context,
		message,
		meta,
	};
	emitStructured(record);
}
