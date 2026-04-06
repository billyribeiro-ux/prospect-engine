import gsap from "gsap";
import type { Action } from "svelte/action";
import { prefersReducedMotion } from "../utils/reducedMotion";

export interface MagneticParams {
	readonly strength?: number;
}

export const magnetic: Action<HTMLElement, MagneticParams | undefined> = (node, params = {}) => {
	const strength = params.strength ?? 0.15;

	if (prefersReducedMotion()) {
		return {};
	}

	function onMove(e: MouseEvent) {
		const rect = node.getBoundingClientRect();
		const cx = rect.left + rect.width / 2;
		const cy = rect.top + rect.height / 2;
		const dx = (e.clientX - cx) * strength;
		const dy = (e.clientY - cy) * strength;
		gsap.to(node, { x: dx, y: dy, duration: 0.2, ease: "power2.out" });
	}

	function onLeave() {
		gsap.to(node, { x: 0, y: 0, duration: 0.35, ease: "elastic.out(1, 0.5)" });
	}

	node.addEventListener("mousemove", onMove);
	node.addEventListener("mouseleave", onLeave);

	return {
		destroy() {
			node.removeEventListener("mousemove", onMove);
			node.removeEventListener("mouseleave", onLeave);
			gsap.killTweensOf(node);
		},
	};
};
