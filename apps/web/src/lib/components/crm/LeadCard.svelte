<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	title: string;
	subtitle?: string;
	children?: Snippet;
}

let { title, subtitle = "", children }: Props = $props();
</script>

<div class="pe-lead-card">
	<div class="pe-lead-card__header">
		<h3 class="pe-lead-card__title">{title}</h3>
		{#if subtitle}
			<p class="pe-lead-card__subtitle">{subtitle}</p>
		{/if}
	</div>
	{#if children}{@render children()}{/if}
</div>

<style>
	@layer components {
		.pe-lead-card {
			padding: var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-lead-card__title {
			margin: 0 0 var(--pe-space-1);
			font-size: var(--pe-font-size-md);
			font-weight: 600;
		}

		.pe-lead-card__subtitle {
			margin: 0;
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}
	}
</style>
