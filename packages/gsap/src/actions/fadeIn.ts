import gsap from "gsap";
import type { Action } from "svelte/action";
import { animationConfig } from "../registry";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface FadeInParams {
	readonly duration?: number;
	readonly delay?: number;
	readonly y?: number;
	readonly ease?: string;
}

export const fadeIn: Action<HTMLElement, FadeInParams | undefined> = (node, params = {}) => {
	const duration = params.duration ?? animationConfig.fadeIn.duration;
	const delay = params.delay ?? 0;
	const y = params.y ?? animationConfig.fadeIn.y;
	const ease = params.ease ?? animationConfig.fadeIn.ease;

	if (prefersReducedMotion()) {
		gsap.set(node, { opacity: 1 });
		return {};
	}

	gsap.fromTo(node, { opacity: 0, y }, { opacity: 1, y: 0, duration, delay, ease });

	return {
		destroy() {
			gsap.killTweensOf(node);
		},
	};
};
