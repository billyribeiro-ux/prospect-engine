<script lang="ts">
import { messages } from "$lib/i18n/messages/en";
</script>

<svelte:head>
	<title>{messages.app.auth.registerTitle} · {messages.app.productName}</title>
</svelte:head>

<h1 class="auth-title">{messages.app.auth.registerTitle}</h1>
<form class="auth-form" method="post" action="/api/v1/auth/register">
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
			autocomplete="new-password"
			required
		/>
	</label>
	<button class="auth-submit" type="submit">{messages.app.auth.submitRegister}</button>
</form>
<p class="auth-footer">
	<a class="auth-link" href="/login">{messages.app.auth.signInLink}</a>
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
