<script lang="ts">
import SendDialog from "$lib/components/email/SendDialog.svelte";
import TemplateEditor from "$lib/components/email/TemplateEditor.svelte";
import TrackingDashboard from "$lib/components/email/TrackingDashboard.svelte";
import { messages } from "$lib/i18n/messages/en";

let template = $state("Hi {{name}},\n\nThanks for visiting {{business}}.\n");
let sendOpen = $state(false);
let to = $state("");
let subject = $state("");
let body = $state("");
let sendLoading = $state(false);
let sendErr = $state("");
let sendOk = $state("");

const metrics = [
	{ id: "opens", label: "Opens (7d)", value: "—" },
	{ id: "clicks", label: "Clicks (7d)", value: "—" },
	{ id: "replies", label: "Replies", value: "—" },
] as const;

async function onSend(e: SubmitEvent): Promise<void> {
	e.preventDefault();
	sendLoading = true;
	sendErr = "";
	sendOk = "";
	try {
		const res = await fetch("/api/v1/email/send", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				to: to.trim(),
				subject: subject.trim(),
				body: body.trim(),
			}),
		});
		const data = (await res.json().catch(() => ({}))) as {
			error?: string;
			status?: string;
			mode?: string;
			delivery?: string;
		};
		if (!res.ok) {
			sendErr = data.error ?? "Send failed";
			return;
		}
		sendOk =
			data.mode === "smtp"
				? `Queued (${data.delivery ?? "smtp"}). Check server logs for delivery.`
				: "Accepted (stub mode — configure SMTP in Settings or PE_SMTP_* for live relay).";
	} finally {
		sendLoading = false;
	}
}
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.email}</h1>
	<p class="workspace-page__body">
		Compose outreach templates and send via the API. Configure SMTP under <strong>Settings</strong> or set
		<code class="workspace-page__code">PE_SMTP_*</code> on the server to relay mail; otherwise sends are logged
		only (stub).
	</p>

	<TemplateEditor bind:value={template} />

	<div class="workspace-page__actions">
		<button type="button" class="workspace-page__btn" onclick={() => (sendOpen = true)}>
			Open send dialog
		</button>
	</div>

	<h2 class="workspace-page__h2">Tracking</h2>
	<TrackingDashboard metrics={[...metrics]} />

	<SendDialog open={sendOpen} onClose={() => (sendOpen = false)}>
		<form class="send-form" onsubmit={onSend}>
			<label class="send-form__label">
				<span>To</span>
				<input class="send-form__input" type="email" bind:value={to} required autocomplete="email" />
			</label>
			<label class="send-form__label">
				<span>Subject</span>
				<input class="send-form__input" type="text" bind:value={subject} required />
			</label>
			<label class="send-form__label">
				<span>Body</span>
				<textarea class="send-form__textarea" rows={6} bind:value={body} required></textarea>
			</label>
			{#if sendErr}
				<p class="send-form__err" role="alert">{sendErr}</p>
			{/if}
			{#if sendOk}
				<p class="send-form__ok" role="status">{sendOk}</p>
			{/if}
			<button class="send-form__submit" type="submit" disabled={sendLoading}>
				{sendLoading ? "Sending…" : "Send"}
			</button>
		</form>
	</SendDialog>
</div>

<style>
	@layer components {
		.workspace-page__title {
			margin: 0;
			font-size: var(--pe-font-size-2xl);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.workspace-page__body {
			margin-block-start: var(--pe-space-3);
			margin-block-end: var(--pe-space-6);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.workspace-page__code {
			font-size: 0.9em;
			padding: 0.1em 0.35em;
			border-radius: var(--pe-radius-sm);
			background: var(--pe-surface-sunken);
		}

		.workspace-page__h2 {
			margin: var(--pe-space-8) 0 var(--pe-space-3);
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.workspace-page__actions {
			margin-block: var(--pe-space-4);
		}

		.workspace-page__btn {
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			padding: var(--pe-space-2) var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
			color: var(--pe-text-primary);
			cursor: pointer;
		}

		.workspace-page__btn:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.send-form {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
			font-family: var(--pe-font-family);
		}

		.send-form__label {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.send-form__input,
		.send-form__textarea {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-overlay);
			color: var(--pe-text-primary);
		}

		.send-form__err {
			margin: 0;
			color: var(--pe-status-error);
			font-size: var(--pe-font-size-sm);
		}

		.send-form__ok {
			margin: 0;
			color: var(--pe-status-success);
			font-size: var(--pe-font-size-sm);
		}

		.send-form__submit {
			align-self: flex-start;
			padding: var(--pe-space-2) var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: none;
			background: var(--pe-interactive-primary);
			color: var(--pe-text-inverse);
			font-weight: 600;
			cursor: pointer;
		}

		.send-form__submit:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}
	}
</style>
