import gsap from "gsap";
import type { Action } from "svelte/action";
import { animationConfig } from "../registry";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface CounterParams {
	readonly from?: number;
	readonly to: number;
	readonly duration?: number;
}

export const counter: Action<HTMLElement, CounterParams | undefined> = (node, params) => {
	if (!params || params.to === undefined) {
		return {};
	}
	const from = params.from ?? 0;
	const to = params.to;
	const duration = params.duration ?? animationConfig.counter.duration;

	if (prefersReducedMotion()) {
		node.textContent = String(to);
		return {};
	}

	const obj = { v: from };
	const tween = gsap.to(obj, {
		v: to,
		duration,
		ease: animationConfig.counter.ease,
		onUpdate: () => {
			node.textContent = String(Math.round(obj.v * 100) / 100);
		},
	});

	return {
		destroy() {
			tween.kill();
		},
	};
};
