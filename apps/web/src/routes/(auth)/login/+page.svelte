<script lang="ts">
import type { AuthSuccess } from "@pe/types/auth";
import { goto } from "$app/navigation";
import { setAuthSession } from "$lib/auth/token";
import { messages } from "$lib/i18n/messages/en";

let loading = $state(false);
let err = $state("");

async function onsubmit(e: SubmitEvent): Promise<void> {
	e.preventDefault();
	const form = e.currentTarget as HTMLFormElement;
	const fd = new FormData(form);
	loading = true;
	err = "";
	try {
		const res = await fetch("/api/v1/auth/login", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				email: String(fd.get("email") ?? ""),
				password: String(fd.get("password") ?? ""),
			}),
		});
		const data = (await res.json().catch(() => ({}))) as { error?: string } & Partial<AuthSuccess>;
		if (!res.ok) {
			err = data.error ?? "Login failed";
			return;
		}
		if (data.token && data.refresh_token != null && data.expires_in != null) {
			setAuthSession(data.token, data.refresh_token, data.expires_in);
		}
		await goto("/discover");
	} finally {
		loading = false;
	}
}
</script>

<svelte:head>
	<title>{messages.app.auth.loginTitle} · {messages.app.productName}</title>
</svelte:head>

<h1 class="auth-title">{messages.app.auth.loginTitle}</h1>
{#if err}
	<p class="auth-error" role="alert">{err}</p>
{/if}
<form class="auth-form" onsubmit={onsubmit}>
	<label class="auth-label">
		<span>{messages.app.auth.emailLabel}</span>
		<input class="auth-input" type="email" name="email" autocomplete="email" required />
	</label>
	<label class="auth-label">
		<span>{messages.app.auth.passwordLabel}</span>
		<input
			class="auth-input"
			type="password"
			name="password"
			autocomplete="current-password"
			required
		/>
	</label>
	<button class="auth-submit" type="submit" disabled={loading}>
		{loading ? "…" : messages.app.auth.submitLogin}
	</button>
</form>
<p class="auth-footer">
	<a class="auth-link" href="/register">{messages.app.auth.registerLink}</a>
	&nbsp;·&nbsp;
	<a class="auth-link" href="/">{messages.app.auth.backHome}</a>
</p>

<style>
	@layer components {
		.auth-title {
			margin: 0 0 var(--pe-space-6);
			font-size: var(--pe-font-size-xl);
			font-weight: 600;
		}

		.auth-error {
			margin: 0 0 var(--pe-space-3);
			color: var(--pe-status-error);
			font-size: var(--pe-font-size-sm);
		}

		.auth-form {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-4);
		}

		.auth-label {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.auth-input {
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-3);
			padding-block: var(--pe-space-2);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-overlay);
			color: var(--pe-text-primary);
		}

		.auth-submit {
			min-block-size: var(--pe-density-row-height);
			margin-block-start: var(--pe-space-2);
			border: none;
			border-radius: var(--pe-radius-md);
			background: var(--pe-interactive-primary);
			color: var(--pe-text-inverse);
			font-weight: 600;
			cursor: pointer;
		}

		.auth-submit:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}

		.auth-submit:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.auth-footer {
			margin-block-start: var(--pe-space-6);
			margin-block-end: 0;
			text-align: center;
		}

		.auth-link {
			color: var(--pe-interactive-primary);
			font-size: var(--pe-font-size-sm);
		}
	}
</style>
