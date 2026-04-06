import gsap from "gsap";
import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface DrawSVGParams {
	readonly duration?: number;
}

export const drawSVG: Action<SVGPathElement, DrawSVGParams | undefined> = (node, params = {}) => {
	const duration = params.duration ?? 1.2;
	const len = node.getTotalLength();

	if (prefersReducedMotion()) {
		gsap.set(node, { strokeDasharray: len, strokeDashoffset: 0 });
		return {};
	}

	gsap.set(node, { strokeDasharray: len, strokeDashoffset: len });
	gsap.to(node, { strokeDashoffset: 0, duration, ease: "power2.inOut" });

	return {
		destroy() {
			gsap.killTweensOf(node);
		},
	};
};
