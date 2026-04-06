<script lang="ts">
import { onMount } from "svelte";
import { browser } from "$app/environment";
import { getToken, refreshAccessToken, shouldRefreshAccessToken } from "$lib/auth/token";

type SmtpSettingsResponse = {
	enabled: boolean;
	host: string;
	port: number;
	username: string | null;
	from: string;
	has_password: boolean;
	active_source: "database" | "environment" | "none";
	environment: {
		configured: boolean;
		host: string | null;
		port: number | null;
	};
};

interface Props {
	labels: {
		sectionTitle: string;
		sectionLead: string;
		activeSource: string;
		sourceDatabase: string;
		sourceEnvironment: string;
		sourceNone: string;
		envHint: string;
		enabled: string;
		host: string;
		port: string;
		username: string;
		password: string;
		passwordKeep: string;
		from: string;
		save: string;
		loadError: string;
		saveError: string;
		signInHint: string;
	};
}

let { labels }: Props = $props();

let loading = $state(true);
let saving = $state(false);
let loadErr = $state("");
let saveErr = $state("");
let data = $state<SmtpSettingsResponse | null>(null);

let enabled = $state(false);
let host = $state("");
let port = $state(587);
let username = $state("");
let password = $state("");
let from = $state("");

async function authHeaders(): Promise<HeadersInit> {
	if (browser && shouldRefreshAccessToken()) {
		await refreshAccessToken();
	}
	const token = getToken();
	const h: Record<string, string> = { "Content-Type": "application/json" };
	if (token) {
		h.Authorization = `Bearer ${token}`;
	}
	return h;
}

function applyResponse(r: SmtpSettingsResponse): void {
	data = r;
	enabled = r.enabled;
	host = r.host;
	port = r.port;
	username = r.username ?? "";
	from = r.from;
	password = "";
}

async function load(): Promise<void> {
	if (!browser) return;
	loading = true;
	loadErr = "";
	try {
		let res = await fetch("/api/v1/settings/smtp", {
			headers: await authHeaders(),
		});
		if (res.status === 401) {
			await refreshAccessToken();
			res = await fetch("/api/v1/settings/smtp", {
				headers: await authHeaders(),
			});
		}
		if (res.status === 401) {
			data = null;
			loadErr = "";
			return;
		}
		if (!res.ok) {
			const j = (await res.json().catch(() => ({}))) as { error?: string };
			loadErr = j.error ?? labels.loadError;
			return;
		}
		const j = (await res.json()) as SmtpSettingsResponse;
		applyResponse(j);
	} finally {
		loading = false;
	}
}

async function save(e: Event): Promise<void> {
	e.preventDefault();
	saveErr = "";
	saving = true;
	try {
		const body: Record<string, unknown> = {
			enabled,
			host: host.trim(),
			port,
			username: username.trim() || null,
			from: from.trim(),
		};
		if (password.trim() !== "") {
			body.password = password.trim();
		}
		const res = await fetch("/api/v1/settings/smtp", {
			method: "PUT",
			headers: await authHeaders(),
			body: JSON.stringify(body),
		});
		const j = (await res.json().catch(() => ({}))) as { error?: string };
		if (!res.ok) {
			saveErr = j.error ?? labels.saveError;
			return;
		}
		applyResponse(j as SmtpSettingsResponse);
	} finally {
		saving = false;
	}
}

onMount(() => {
	if (browser) {
		void load();
	}
});

function sourceLabel(s: SmtpSettingsResponse["active_source"]): string {
	if (s === "database") return labels.sourceDatabase;
	if (s === "environment") return labels.sourceEnvironment;
	return labels.sourceNone;
}
</script>

<section class="smtp-settings" aria-labelledby="smtp-settings-heading">
	<h2 id="smtp-settings-heading" class="smtp-settings__title">{labels.sectionTitle}</h2>
	<p class="smtp-settings__lead">{labels.sectionLead}</p>

	{#if loading}
		<p class="smtp-settings__muted">Loading…</p>
	{:else if loadErr}
		<p class="smtp-settings__err" role="alert">{loadErr}</p>
	{:else if data === null}
		<p class="smtp-settings__muted">{labels.signInHint}</p>
	{:else}
		<p class="smtp-settings__meta">
			<span class="smtp-settings__meta-label">{labels.activeSource}</span>
			<strong>{sourceLabel(data.active_source)}</strong>
		</p>
		{#if data.environment.configured}
			<p class="smtp-settings__muted">
				{labels.envHint}
				{#if data.environment.host}
					<code class="smtp-settings__code">{data.environment.host}</code>
					{#if data.environment.port != null}
						:{data.environment.port}
					{/if}
				{/if}
			</p>
		{/if}

		<form class="smtp-settings__form" onsubmit={save}>
			<label class="smtp-settings__check">
				<input type="checkbox" bind:checked={enabled} />
				{labels.enabled}
			</label>

			<label class="smtp-settings__label">
				<span>{labels.host}</span>
				<input class="smtp-settings__input" type="text" bind:value={host} autocomplete="off" />
			</label>

			<label class="smtp-settings__label">
				<span>{labels.port}</span>
				<input
					class="smtp-settings__input"
					type="number"
					min="1"
					max="65535"
					bind:value={port}
				/>
			</label>

			<label class="smtp-settings__label">
				<span>{labels.username}</span>
				<input
					class="smtp-settings__input"
					type="text"
					bind:value={username}
					autocomplete="username"
				/>
			</label>

			<label class="smtp-settings__label">
				<span>{labels.password}</span>
				<input
					class="smtp-settings__input"
					type="password"
					bind:value={password}
					autocomplete="new-password"
					placeholder={data.has_password ? labels.passwordKeep : ""}
				/>
			</label>

			<label class="smtp-settings__label">
				<span>{labels.from}</span>
				<input
					class="smtp-settings__input"
					type="text"
					bind:value={from}
					placeholder="noreply@example.com"
				/>
			</label>

			{#if saveErr}
				<p class="smtp-settings__err" role="alert">{saveErr}</p>
			{/if}

			<button type="submit" class="smtp-settings__btn" disabled={saving}>
				{saving ? "Saving…" : labels.save}
			</button>
		</form>
	{/if}
</section>

<style>
	@layer components {
		.smtp-settings__title {
			margin: 0;
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.smtp-settings__lead {
			margin-block: var(--pe-space-2) var(--pe-space-4);
			max-inline-size: 42rem;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.smtp-settings__muted {
			margin: 0;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.smtp-settings__meta {
			margin: 0 0 var(--pe-space-3);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.smtp-settings__meta-label {
			margin-inline-end: var(--pe-space-2);
		}

		.smtp-settings__code {
			font-size: 0.9em;
		}

		.smtp-settings__form {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
			max-inline-size: 28rem;
		}

		.smtp-settings__check {
			display: flex;
			align-items: center;
			gap: var(--pe-space-2);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-primary);
		}

		.smtp-settings__label {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.smtp-settings__input {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
			font: inherit;
		}

		.smtp-settings__btn {
			align-self: flex-start;
			padding: var(--pe-space-2) var(--pe-space-4);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-interactive-primary);
			color: var(--pe-text-inverse);
			font: inherit;
			font-weight: 500;
			cursor: pointer;
		}

		.smtp-settings__btn:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}

		.smtp-settings__err {
			margin: 0;
			font-size: var(--pe-density-font-size);
			color: var(--pe-status-error);
		}
	}
</style>
