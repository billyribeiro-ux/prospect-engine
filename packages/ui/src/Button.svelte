<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	type?: "button" | "submit" | "reset";
	variant?: "primary" | "ghost" | "danger";
	disabled?: boolean;
	children: Snippet;
}

let { type = "button", variant = "primary", disabled = false, children }: Props = $props();
</script>

<button class="pe-button" class:pe-button--ghost={variant === "ghost"} class:pe-button--danger={variant === "danger"} {type} {disabled}>{@render children()}</button>

<style>
	@layer components {
		.pe-button {
			--_bg: var(--pe-interactive-primary);
			--_color: var(--pe-text-inverse);
			--_radius: var(--pe-radius-md);
			--_height: var(--pe-density-row-height);

			display: inline-flex;
			align-items: center;
			justify-content: center;
			block-size: var(--_height);
			padding-inline: var(--pe-space-4);
			background: var(--_bg);
			color: var(--_color);
			border-radius: var(--_radius);
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			border: none;
			cursor: pointer;
			transition: background var(--pe-duration-fast) var(--pe-easing-default);
		}

		.pe-button:hover:not(:disabled) {
			--_bg: var(--pe-interactive-hover);
		}

		.pe-button:active:not(:disabled) {
			--_bg: var(--pe-interactive-active);
		}

		.pe-button:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.pe-button:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}

		.pe-button--ghost {
			--_bg: transparent;
			--_color: var(--pe-text-primary);
			border: 1px solid var(--pe-border-default);
		}

		.pe-button--danger {
			--_bg: var(--pe-status-error);
		}
	}
</style>
