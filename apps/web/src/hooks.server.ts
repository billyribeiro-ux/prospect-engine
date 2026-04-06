import type { Handle } from "@sveltejs/kit";
import { parseDensityCookie, parseThemeCookie } from "$lib/constants/theme";

export const handle: Handle = async ({ event, resolve }) => {
	const theme = parseThemeCookie(event.cookies.get("pe_theme"));
	const density = parseDensityCookie(event.cookies.get("pe_density"));
	event.locals.theme = theme;
	event.locals.density = density;

	const response = await resolve(event, {
		transformPageChunk: ({ html }) => {
			return html.replace(
				'<html lang="en">',
				`<html lang="en" data-theme="${theme}" data-density="${density}">`,
			);
		},
	});

	return response;
};
