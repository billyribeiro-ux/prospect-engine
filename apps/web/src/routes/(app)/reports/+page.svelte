<script lang="ts">
import ExportDialog from "$lib/components/reports/ExportDialog.svelte";
import ReportPreview from "$lib/components/reports/ReportPreview.svelte";
import WhiteLabelConfig from "$lib/components/reports/WhiteLabelConfig.svelte";
import { messages } from "$lib/i18n/messages/en";

let exportOpen = $state(false);
let logoUrl = $state("");
let accent = $state("#3b82f6");
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.reports}</h1>
	<p class="workspace-page__body">
		Preview, branding, and export flows use sample bindings until report generation is implemented server-side.
	</p>

	<div class="reports-actions">
		<button type="button" class="reports-actions__btn" onclick={() => (exportOpen = true)}>
			Open export dialog
		</button>
	</div>

	<ReportPreview title="Executive summary">
		<p class="workspace-page__preview-copy">
			Quarterly roll-up of discovery, audit scores, and pipeline movement will render here.
		</p>
	</ReportPreview>

	<h2 class="workspace-page__h2">White-label</h2>
	<WhiteLabelConfig bind:logoUrl bind:accent />

	<ExportDialog open={exportOpen} onClose={() => (exportOpen = false)}>
		<p class="workspace-page__preview-copy">PDF and CSV export will call the API from this dialog.</p>
	</ExportDialog>
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
			margin-block-end: var(--pe-space-4);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.workspace-page__h2 {
			margin: var(--pe-space-8) 0 var(--pe-space-3);
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.workspace-page__preview-copy {
			margin: 0;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.reports-actions {
			margin-block-end: var(--pe-space-6);
		}

		.reports-actions__btn {
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			padding: var(--pe-space-2) var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
			color: var(--pe-text-primary);
			cursor: pointer;
		}

		.reports-actions__btn:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}
	}
</style>
