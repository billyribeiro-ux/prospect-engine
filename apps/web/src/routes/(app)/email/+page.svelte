<script lang="ts">
import SendDialog from "$lib/components/email/SendDialog.svelte";
import TemplateEditor from "$lib/components/email/TemplateEditor.svelte";
import TrackingDashboard from "$lib/components/email/TrackingDashboard.svelte";
import { messages } from "$lib/i18n/messages/en";

let template = $state("Hi {{name}},\n\nThanks for visiting {{business}}.\n");
let sendOpen = $state(false);

const metrics = [
	{ id: "opens", label: "Opens (7d)", value: "—" },
	{ id: "clicks", label: "Clicks (7d)", value: "—" },
	{ id: "replies", label: "Replies", value: "—" },
] as const;
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.email}</h1>
	<p class="workspace-page__body">
		Compose outreach templates and review tracking metrics. Sending is wired to the API in a later phase.
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
		<p class="workspace-page__muted">SMTP and provider hooks will connect here.</p>
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

		.workspace-page__muted {
			margin: 0;
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}
	}
</style>
