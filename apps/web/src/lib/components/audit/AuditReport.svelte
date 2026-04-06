<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	title?: string;
	children?: Snippet;
}

let { title = "Audit report", children }: Props = $props();
</script>

<article class="pe-audit-report" aria-labelledby="pe-audit-heading">
	<h2 id="pe-audit-heading" class="pe-audit-report__title">{title}</h2>
	{#if children}{@render children()}{/if}
</article>

<style>
	@layer components {
		.pe-audit-report {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-4);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-audit-report__title {
			margin: 0;
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
		}
	}
</style>
