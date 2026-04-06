<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	label: string;
	children?: Snippet;
}

let { label, children }: Props = $props();

let visible = $state(false);
</script>

<span
	class="pe-tooltip"
	role="group"
	onmouseenter={() => {
		visible = true;
	}}
	onmouseleave={() => {
		visible = false;
	}}
	onfocusin={() => {
		visible = true;
	}}
	onfocusout={() => {
		visible = false;
	}}
>
	{#if children}{@render children()}{/if}
	{#if visible}
		<span class="pe-tooltip__bubble" role="tooltip">{label}</span>
	{/if}
</span>

<style>
	@layer components {
		.pe-tooltip {
			position: relative;
			display: inline-flex;
		}

		.pe-tooltip__bubble {
			position: absolute;
			inset-block-end: 100%;
			inset-inline-start: 50%;
			translate: -50% -0.25rem;
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-xs);
			white-space: nowrap;
			background: var(--pe-color-gray-900);
			color: var(--pe-text-inverse);
			box-shadow: var(--pe-shadow-md);
			z-index: var(--pe-z-overlay);
		}
	}
</style>
