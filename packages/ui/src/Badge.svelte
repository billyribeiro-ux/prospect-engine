<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	variant?: "default" | "success" | "warning" | "error";
	children?: Snippet;
}

let { variant = "default", children }: Props = $props();
</script>

<span class="pe-badge" class:pe-badge--success={variant === "success"} class:pe-badge--warning={variant === "warning"} class:pe-badge--error={variant === "error"}>
	{#if children}{@render children()}{/if}
</span>

<style>
	@layer components {
		.pe-badge {
			display: inline-flex;
			align-items: center;
			padding-inline: var(--pe-space-2);
			padding-block: var(--pe-space-1);
			border-radius: var(--pe-radius-sm);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-xs);
			font-weight: 600;
			background: var(--pe-surface-raised);
			color: var(--pe-text-primary);
			border: 1px solid var(--pe-border-default);
		}

		.pe-badge--success {
			color: var(--pe-status-success);
			border-color: color-mix(in srgb, var(--pe-status-success) 40%, transparent);
		}

		.pe-badge--warning {
			color: var(--pe-status-warning);
			border-color: color-mix(in srgb, var(--pe-status-warning) 40%, transparent);
		}

		.pe-badge--error {
			color: var(--pe-status-error);
			border-color: color-mix(in srgb, var(--pe-status-error) 40%, transparent);
		}
	}
</style>
