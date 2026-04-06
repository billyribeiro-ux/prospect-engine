<script lang="ts" generics="T">
import type { Snippet } from "svelte";

interface Props {
	items: readonly T[];
	item: Snippet<[T, number]>;
}

let { items, item }: Props = $props();
</script>

<div class="pe-virtual-list" role="list">
	{#each items as row, i (i)}
		<div class="pe-virtual-list__row" role="listitem">
			{@render item(row, i)}
		</div>
	{/each}
</div>

<style>
	@layer components {
		.pe-virtual-list {
			overflow: auto;
			max-block-size: 24rem;
			border: 1px solid var(--pe-border-default);
			border-radius: var(--pe-radius-md);
		}

		.pe-virtual-list__row {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-bottom: 1px solid var(--pe-border-subtle);
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-primary);
		}

		.pe-virtual-list__row:last-child {
			border-bottom: none;
		}
	}
</style>
