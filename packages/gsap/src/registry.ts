export const animationConfig = {
	fadeIn: { duration: 0.5, ease: "power2.out", y: 20 },
	stagger: { each: 0.08, ease: "power2.out", y: 15 },
	counter: { duration: 1.2, ease: "power2.out" },
	scoreReveal: {
		ring: { duration: 1.2, ease: "power3.out" },
		bars: { stagger: 0.1, duration: 0.6, ease: "back.out(1.2)" },
		grade: { duration: 0.4, ease: "back.out(2)", scale: { from: 0, to: 1 } },
		total: 1.8,
	},
	crawlVisualizer: {
		nodeEntrance: { duration: 0.3, ease: "power2.out", scale: { from: 0, to: 1 } },
		connectionDraw: { duration: 0.4, ease: "none" },
		pulseLoop: {
			duration: 0.8,
			repeat: -1,
			yoyo: true,
			opacity: { min: 0.4, max: 1 },
		},
		successFlash: { duration: 0.2, ease: "power1.in" },
		errorShake: { duration: 0.4, x: [-5, 5, -3, 3, 0], ease: "power1.out" },
	},
	mapOverlay: {
		markerDrop: { duration: 0.5, ease: "bounce.out", y: { from: -50, to: 0 } },
		markerStagger: 0.03,
		heatmapWipe: { duration: 1.0, ease: "power2.out" },
		clusterExpand: { duration: 0.3, ease: "back.out(1.5)" },
	},
	pageTransition: { duration: 0.25, ease: "power2.inOut" },
	microInteraction: { duration: 0.15, ease: "power1.out" },
} as const;
