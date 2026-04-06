<script lang="ts">
import type { Snippet } from "svelte";

interface Column {
	readonly id: string;
	readonly title: string;
}

interface Props {
	columns?: readonly Column[];
	children?: Snippet;
}

let { columns = [], children }: Props = $props();
</script>

<div class="pe-kanban" role="region" aria-label="Pipeline board">
	<div class="pe-kanban__columns">
		{#each columns as col (col.id)}
			<section class="pe-kanban__column" aria-labelledby={`col-${col.id}`}>
				<h3 id={`col-${col.id}`} class="pe-kanban__column-title">{col.title}</h3>
			</section>
		{/each}
	</div>
	{#if children}{@render children()}{/if}
</div>

<style>
	@layer components {
		.pe-kanban {
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-kanban__columns {
			display: flex;
			gap: var(--pe-space-4);
			overflow-x: auto;
		}

		.pe-kanban__column {
			min-inline-size: 14rem;
			padding: var(--pe-space-3);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-sunken);
		}

		.pe-kanban__column-title {
			margin: 0 0 var(--pe-space-2);
			font-size: var(--pe-font-size-sm);
			font-weight: 600;
		}
	}
</style>
