import gsap from "gsap";
import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface MorphPathParams {
	readonly duration?: number;
}

/** Stand-in until MorphSVGPlugin: opacity fade. */
export const morphPath: Action<HTMLElement, MorphPathParams | undefined> = (node, params = {}) => {
	const duration = params.duration ?? 0.45;

	if (prefersReducedMotion()) {
		gsap.set(node, { opacity: 1 });
		return {};
	}

	gsap.fromTo(node, { opacity: 0 }, { opacity: 1, duration, ease: "power2.out" });

	return {
		destroy() {
			gsap.killTweensOf(node);
		},
	};
};
