<script lang="ts">
import AuditReport from "$lib/components/audit/AuditReport.svelte";
import CompetitorBench from "$lib/components/audit/CompetitorBench.svelte";
import DimensionCard from "$lib/components/audit/DimensionCard.svelte";
import ScoreRing from "$lib/components/audit/ScoreRing.svelte";
import { messages } from "$lib/i18n/messages/en";

const competitors = [
	{ name: "Local rival A", score: 78 },
	{ name: "Local rival B", score: 71 },
] as const;
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.audit}</h1>
	<p class="workspace-page__body">
		Dimension scores and benchmarks below use sample data until the audit engine is wired to the API.
	</p>

	<AuditReport title="Latest audit">
		<div class="audit-layout">
			<div class="audit-layout__ring">
				<ScoreRing score={72} max={100} size={120} />
			</div>
			<div class="audit-layout__dims">
				<DimensionCard label="Performance" score={80} />
				<DimensionCard label="SEO" score={74} />
				<DimensionCard label="Accessibility" score={68} />
			</div>
			<CompetitorBench rows={[...competitors]} />
		</div>
	</AuditReport>
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

		.audit-layout {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-6);
		}

		.audit-layout__ring {
			display: flex;
			justify-content: center;
		}

		.audit-layout__dims {
			display: grid;
			grid-template-columns: repeat(auto-fill, minmax(10rem, 1fr));
			gap: var(--pe-space-3);
		}
	}
</style>
