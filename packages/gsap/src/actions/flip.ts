import gsap from "gsap";
import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface FlipParams {
	readonly duration?: number;
}

/** Layout transition stand-in (not GSAP Flip plugin). */
export const flip: Action<HTMLElement, FlipParams | undefined> = (node, params = {}) => {
	const duration = params.duration ?? 0.35;

	if (prefersReducedMotion()) {
		gsap.set(node, { scale: 1, opacity: 1 });
		return {};
	}

	gsap.fromTo(
		node,
		{ scale: 0.94, opacity: 0.5 },
		{ scale: 1, opacity: 1, duration, ease: "back.out(1.4)" },
	);

	return {
		destroy() {
			gsap.killTweensOf(node);
		},
	};
};
