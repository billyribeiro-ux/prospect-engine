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

let url = $state("");
let loading = $state(false);
let err = $state("");
let gradedAt = $state<string | null>(null);
let composite = $state<number | null>(null);
let dimensions = $state<{ dimension: string; score: number }[]>([]);

function titleCaseDim(id: string): string {
	return id.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
}

async function onAuditSubmit(e: SubmitEvent): Promise<void> {
	e.preventDefault();
	const u = url.trim();
	if (!u) {
		err = "Enter a page URL to audit.";
		return;
	}
	loading = true;
	err = "";
	gradedAt = null;
	composite = null;
	dimensions = [];
	try {
		const res = await fetch(`/api/v1/audit?url=${encodeURIComponent(u)}`);
		const data = (await res.json().catch(() => ({}))) as {
			error?: string;
			status?: string;
			composite?: number;
			dimensions?: { dimension: string; score: number }[];
			gradedAt?: string;
		};
		if (!res.ok) {
			err = data.error ?? "Audit request failed";
			return;
		}
		if (data.status === "hint") {
			err = "Unexpected hint response";
			return;
		}
		if (data.composite == null || !data.dimensions) {
			err = "Invalid audit response";
			return;
		}
		composite = data.composite;
		dimensions = data.dimensions.map((d) => ({
			dimension: d.dimension,
			score: Math.round(d.score),
		}));
		gradedAt = data.gradedAt ?? null;
	} finally {
		loading = false;
	}
}
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.audit}</h1>
	<p class="workspace-page__body">
		Run a heuristic audit against any public URL (HTML fetched server-side). Benchmarks below remain sample
		data for positioning.
	</p>

	<form class="audit-run" onsubmit={onAuditSubmit}>
		<label class="audit-run__label">
			<span>Page URL</span>
			<input
				class="audit-run__input"
				type="url"
				name="url"
				placeholder="https://example.com"
				bind:value={url}
				required
			/>
		</label>
		<button class="audit-run__btn" type="submit" disabled={loading}>
			{loading ? "Running…" : "Run audit"}
		</button>
	</form>
	{#if err}
		<p class="audit-run__err" role="alert">{err}</p>
	{/if}

	<AuditReport title="Latest audit">
		{#if composite != null}
			<p class="audit-meta">
				{#if gradedAt}
					Graded at {gradedAt}
				{/if}
			</p>
			<div class="audit-layout">
				<div class="audit-layout__ring">
					<ScoreRing score={composite} max={100} size={120} />
				</div>
				<div class="audit-layout__dims">
					{#each dimensions.slice(0, 7) as d (d.dimension)}
						<DimensionCard label={titleCaseDim(d.dimension)} score={d.score} />
					{/each}
				</div>
				<CompetitorBench rows={[...competitors]} />
			</div>
		{:else}
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
			<p class="audit-placeholder">Run an audit above to replace sample scores with live results.</p>
		{/if}
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

		.audit-run {
			display: flex;
			flex-wrap: wrap;
			gap: var(--pe-space-3);
			align-items: flex-end;
			margin-block-end: var(--pe-space-4);
		}

		.audit-run__label {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
			flex: 1;
			min-inline-size: 12rem;
		}

		.audit-run__input {
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-3);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-overlay);
			color: var(--pe-text-primary);
		}

		.audit-run__btn {
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: none;
			background: var(--pe-interactive-primary);
			color: var(--pe-text-inverse);
			font-weight: 600;
			cursor: pointer;
		}

		.audit-run__btn:disabled {
			opacity: 0.6;
			cursor: not-allowed;
		}

		.audit-run__err {
			margin: 0 0 var(--pe-space-4);
			color: var(--pe-status-error);
			font-size: var(--pe-font-size-sm);
		}

		.audit-meta {
			margin: 0 0 var(--pe-space-3);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.audit-placeholder {
			margin-block-start: var(--pe-space-4);
			font-size: var(--pe-font-size-sm);
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
