import { parseDensityCookie, parseThemeCookie } from "$lib/constants/theme";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies, depends }) => {
	depends("app:theme");
	const theme = parseThemeCookie(cookies.get("pe_theme"));
	const density = parseDensityCookie(cookies.get("pe_density"));
	return { theme, density };
};
