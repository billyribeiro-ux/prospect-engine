import type { KeyboardMode, ShortcutAction, ShortcutDefinition } from "@pe/types/keyboard";

class KeyboardManager {
	mode = $state<KeyboardMode>("palette");
	vimMode = $state<"normal" | "insert" | "visual">("normal");

	private shortcuts = $state<Map<string, ShortcutAction>>(new Map());

	register(definition: ShortcutDefinition, action: ShortcutAction): void {
		const next = new Map(this.shortcuts);
		next.set(definition.id, action);
		this.shortcuts = next;
	}

	unregister(id: string): void {
		const next = new Map(this.shortcuts);
		next.delete(id);
		this.shortcuts = next;
	}
}

export const keyboardManager = new KeyboardManager();
