<script lang="ts">
interface Entry {
	readonly id: string;
	readonly title: string;
	readonly at: string;
}

interface Props {
	entries?: readonly Entry[];
}

let { entries = [] }: Props = $props();
</script>

<ol class="pe-activity-timeline" aria-label="Activity">
	{#each entries as e (e.id)}
		<li class="pe-activity-timeline__item">
			<span class="pe-activity-timeline__dot" aria-hidden="true"></span>
			<div class="pe-activity-timeline__body">
				<p class="pe-activity-timeline__title">{e.title}</p>
				<time class="pe-activity-timeline__time" datetime={e.at}>{e.at}</time>
			</div>
		</li>
	{/each}
</ol>

<style>
	@layer components {
		.pe-activity-timeline {
			margin: 0;
			padding: 0;
			list-style: none;
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-activity-timeline__item {
			display: grid;
			grid-template-columns: auto 1fr;
			gap: var(--pe-space-3);
			padding-block: var(--pe-space-2);
			border-left: 2px solid var(--pe-border-subtle);
			padding-inline-start: var(--pe-space-4);
			position: relative;
		}

		.pe-activity-timeline__dot {
			position: absolute;
			inset-inline-start: -5px;
			inset-block-start: 0.5rem;
			inline-size: 8px;
			block-size: 8px;
			border-radius: var(--pe-radius-full);
			background: var(--pe-interactive-primary);
		}

		.pe-activity-timeline__title {
			margin: 0 0 var(--pe-space-1);
			font-size: var(--pe-font-size-sm);
			font-weight: 600;
		}

		.pe-activity-timeline__time {
			font-size: var(--pe-font-size-xs);
			color: var(--pe-text-secondary);
		}
	}
</style>
