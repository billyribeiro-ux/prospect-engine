<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	title?: string;
	children?: Snippet;
}

let { title = "Preview", children }: Props = $props();
</script>

<section class="pe-report-preview" aria-labelledby="pe-report-preview-h">
	<h2 id="pe-report-preview-h" class="pe-report-preview__title">{title}</h2>
	<div class="pe-report-preview__frame">
		{#if children}{@render children()}{/if}
	</div>
</section>

<style>
	@layer components {
		.pe-report-preview {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-report-preview__title {
			margin: 0;
			font-size: var(--pe-font-size-md);
			font-weight: 600;
		}

		.pe-report-preview__frame {
			min-block-size: 8rem;
			padding: var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
		}
	}
</style>
