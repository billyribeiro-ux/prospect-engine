<script lang="ts">
interface Stage {
	readonly id: string;
	readonly label: string;
	readonly count: number;
}

interface Props {
	stages?: readonly Stage[];
}

let { stages = [] }: Props = $props();
</script>

<div class="pe-pipeline-funnel" role="region" aria-label="Pipeline funnel">
	{#each stages as s (s.id)}
		<div class="pe-pipeline-funnel__stage">
			<span class="pe-pipeline-funnel__label">{s.label}</span>
			<span class="pe-pipeline-funnel__count" aria-label={`${s.label} count`}>{s.count}</span>
		</div>
	{/each}
</div>

<style>
	@layer components {
		.pe-pipeline-funnel {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-pipeline-funnel__stage {
			display: flex;
			justify-content: space-between;
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
		}

		.pe-pipeline-funnel__count {
			font-variant-numeric: tabular-nums;
			font-weight: 600;
		}
	}
</style>
