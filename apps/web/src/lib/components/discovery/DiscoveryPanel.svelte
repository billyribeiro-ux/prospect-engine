<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	title?: string;
	children?: Snippet;
}

let { title = "Discovery", children }: Props = $props();
</script>

<section class="pe-discovery-panel" aria-labelledby="pe-discovery-heading">
	<h2 id="pe-discovery-heading" class="pe-discovery-panel__title">{title}</h2>
	{#if children}{@render children()}{/if}
</section>

<style>
	@layer components {
		.pe-discovery-panel {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-4);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-discovery-panel__title {
			margin: 0;
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
		}
	}
</style>
