import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface ScrambleTextParams {
	readonly duration?: number;
}

const CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

export const scrambleText: Action<HTMLElement, ScrambleTextParams | undefined> = (
	node,
	params = {},
) => {
	const duration = params.duration ?? 0.8;
	const finalText = node.textContent ?? "";

	if (prefersReducedMotion()) {
		return {};
	}

	const totalFrames = Math.max(12, Math.floor(duration * 30));
	let frame = 0;
	const id = globalThis.setInterval(() => {
		frame += 1;
		if (frame >= totalFrames) {
			node.textContent = finalText;
			globalThis.clearInterval(id);
			return;
		}
		let out = "";
		for (let i = 0; i < finalText.length; i += 1) {
			const ch = finalText.charAt(i);
			out += ch === " " ? " " : (CHARS[Math.floor(Math.random() * CHARS.length)] ?? "?");
		}
		node.textContent = out;
	}, 1000 / 30);

	return {
		destroy() {
			globalThis.clearInterval(id);
			node.textContent = finalText;
		},
	};
};
