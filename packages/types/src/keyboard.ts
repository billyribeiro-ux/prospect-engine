export type KeyboardMode = "palette" | "vim" | "vscode";

export type VimSubMode = "normal" | "insert" | "visual";

export interface ShortcutDefinition {
	readonly id: string;
	readonly keys: string;
	readonly actionId: string;
	readonly contexts: readonly string[];
}

export type ShortcutAction = () => void;
