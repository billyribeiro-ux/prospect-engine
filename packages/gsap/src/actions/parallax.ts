import gsap from "gsap";
import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface ParallaxParams {
	readonly strength?: number;
}

export const parallax: Action<HTMLElement, ParallaxParams | undefined> = (node, params = {}) => {
	const strength = params.strength ?? 0.08;

	if (prefersReducedMotion()) {
		return {};
	}

	function onScroll() {
		const rect = node.getBoundingClientRect();
		const y = (globalThis.innerHeight / 2 - rect.top) * strength;
		gsap.set(node, { y });
	}

	globalThis.addEventListener("scroll", onScroll, { passive: true });
	onScroll();

	return {
		destroy() {
			globalThis.removeEventListener("scroll", onScroll);
		},
	};
};
