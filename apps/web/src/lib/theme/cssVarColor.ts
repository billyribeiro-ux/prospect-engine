/** How to sample a CSS variable that holds a color (computed style differs by usage). */
export type CssVarProbe = "foreground" | "surface" | "border";

/**
 * Reads the resolved RGB color of `var(--varName)` from the document and returns `#rrggbb`.
 * Used to seed color inputs when there is no custom override.
 */
export function getCssVarAsHex(varName: string, probe: CssVarProbe): string {
	if (typeof document === "undefined") {
		return "#888888";
	}
	const el = document.createElement("div");
	el.style.cssText =
		"position:absolute;visibility:hidden;pointer-events:none;left:0;top:0;width:1px;height:1px;";
	if (probe === "foreground") {
		el.style.color = `var(${varName})`;
	} else if (probe === "surface") {
		el.style.backgroundColor = `var(${varName})`;
	} else {
		el.style.border = `2px solid var(${varName})`;
	}
	document.body.appendChild(el);
	const cs = getComputedStyle(el);
	const rgb =
		probe === "foreground" ? cs.color : probe === "surface" ? cs.backgroundColor : cs.borderColor;
	document.body.removeChild(el);
	return rgbStringToHex(rgb);
}

function rgbStringToHex(rgb: string): string {
	const m = rgb.match(/rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)/);
	if (!m) {
		return "#888888";
	}
	const r = Number(m[1]);
	const g = Number(m[2]);
	const b = Number(m[3]);
	return `#${[r, g, b].map((x) => x.toString(16).padStart(2, "0")).join("")}`;
}
