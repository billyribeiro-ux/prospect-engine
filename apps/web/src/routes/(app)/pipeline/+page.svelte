<script lang="ts">
import ActivityTimeline from "$lib/components/crm/ActivityTimeline.svelte";
import KanbanBoard from "$lib/components/crm/KanbanBoard.svelte";
import LeadCard from "$lib/components/crm/LeadCard.svelte";
import PipelineFunnel from "$lib/components/crm/PipelineFunnel.svelte";
import { messages } from "$lib/i18n/messages/en";

const columns = [
	{ id: "new", title: "New" },
	{ id: "working", title: "Working" },
	{ id: "won", title: "Won" },
] as const;

const stages = [
	{ id: "lead", label: "Lead", count: 24 },
	{ id: "qualified", label: "Qualified", count: 12 },
	{ id: "won", label: "Won", count: 4 },
] as const;

const entries = [
	{ id: "e1", title: "Outbound call — Main St Cafe", at: "2026-04-02" },
	{ id: "e2", title: "Moved to Working", at: "2026-04-01" },
] as const;
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.pipeline}</h1>
	<p class="workspace-page__body">
		Pipeline and CRM views use sample data. Sync with the API will replace fixtures in a later phase.
	</p>

	<PipelineFunnel stages={[...stages]} />

	<KanbanBoard columns={[...columns]}>
		<LeadCard title="Main St Cafe" subtitle="Coffee · 0.8 mi" />
	</KanbanBoard>

	<h2 class="workspace-page__h2">Activity</h2>
	<ActivityTimeline entries={[...entries]} />
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
	}
</style>
