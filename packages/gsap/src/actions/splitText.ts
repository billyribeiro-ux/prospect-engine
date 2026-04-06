import gsap from "gsap";
import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface SplitTextParams {
	readonly duration?: number;
}

export const splitText: Action<HTMLElement, SplitTextParams | undefined> = (node, params = {}) => {
	const duration = params.duration ?? 0.5;
	const text = node.textContent ?? "";

	if (prefersReducedMotion()) {
		return {};
	}

	node.textContent = "";
	const spans: HTMLSpanElement[] = [];
	for (const ch of text) {
		const s = document.createElement("span");
		s.textContent = ch;
		s.style.display = "inline-block";
		node.appendChild(s);
		spans.push(s);
	}

	gsap.fromTo(
		spans,
		{ opacity: 0, y: 6 },
		{ opacity: 1, y: 0, duration, stagger: 0.02, ease: "power2.out" },
	);

	return {
		destroy() {
			gsap.killTweensOf(spans);
			node.textContent = text;
		},
	};
};
