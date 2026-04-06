import { env } from "$env/dynamic/private";
import type { RequestHandler } from "./$types";

function upstreamBase(): string {
	const base = env.PE_API_ORIGIN;
	if (base !== undefined && base.length > 0) {
		return base.replace(/\/$/, "");
	}
	return "http://127.0.0.1:8080";
}

function pathTail(pathParam: string | string[] | undefined): string {
	if (pathParam === undefined) {
		return "";
	}
	return Array.isArray(pathParam) ? pathParam.join("/") : pathParam;
}

async function proxyRequest(event: Parameters<RequestHandler>[0]): Promise<Response> {
	const { request, params } = event;
	const tail = pathTail(params.path);
	const requestUrl = new URL(request.url);
	const target = new URL(`${upstreamBase()}/api/v1/${tail}`);
	target.search = requestUrl.search;

	const headers = new Headers(request.headers);
	headers.delete("host");

	const method = request.method;
	if (method === "GET" || method === "HEAD") {
		return fetch(target, { method, headers });
	}

	const body = await request.arrayBuffer();
	return fetch(target, { method, headers, body });
}

export const GET: RequestHandler = (e) => proxyRequest(e);
export const POST: RequestHandler = (e) => proxyRequest(e);
export const PUT: RequestHandler = (e) => proxyRequest(e);
export const PATCH: RequestHandler = (e) => proxyRequest(e);
export const DELETE: RequestHandler = (e) => proxyRequest(e);
