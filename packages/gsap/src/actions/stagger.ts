import gsap from "gsap";
import type { Action } from "svelte/action";
import { animationConfig } from "../registry";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface StaggerParams {
	readonly stagger?: number;
}

export const stagger: Action<HTMLElement, StaggerParams | undefined> = (node, params = {}) => {
	const each = params.stagger ?? animationConfig.stagger.each;
	const children = [...node.children] as HTMLElement[];

	if (prefersReducedMotion()) {
		for (const el of children) {
			gsap.set(el, { opacity: 1, y: 0 });
		}
		return {};
	}

	children.forEach((el, i) => {
		gsap.fromTo(
			el,
			{ opacity: 0, y: animationConfig.stagger.y },
			{
				opacity: 1,
				y: 0,
				duration: 0.4,
				delay: i * each,
				ease: animationConfig.stagger.ease,
			},
		);
	});

	return {
		destroy() {
			gsap.killTweensOf(children);
		},
	};
};
