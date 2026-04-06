<script lang="ts">
import type { Snippet } from "svelte";

interface Props {
	open?: boolean;
	title?: string;
	onClose?: () => void;
	children?: Snippet;
}

let { open = false, title = "", onClose, children }: Props = $props();

let dialogEl: HTMLDialogElement | undefined = $state();

$effect(() => {
	const el = dialogEl;
	if (!el) return;
	if (open && !el.open) {
		el.showModal();
	} else if (!open && el.open) {
		el.close();
	}
});

function handleClose() {
	onClose?.();
}
</script>

<dialog bind:this={dialogEl} class="pe-modal" aria-labelledby={title ? "pe-modal-title" : undefined} onclose={handleClose}>
	{#if title}
		<h2 id="pe-modal-title" class="pe-modal__title">{title}</h2>
	{/if}
	{#if children}{@render children()}{/if}
</dialog>

<style>
	@layer components {
		.pe-modal {
			min-inline-size: min(90vw, 28rem);
			max-block-size: min(90vh, 40rem);
			overflow: auto;
			padding: var(--pe-space-5);
			border: 1px solid var(--pe-border-default);
			border-radius: var(--pe-radius-lg);
			background: var(--pe-surface-raised);
			box-shadow: var(--pe-shadow-lg);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
			z-index: var(--pe-z-modal);
		}

		.pe-modal::backdrop {
			background: color-mix(in srgb, var(--pe-text-primary) 35%, transparent);
		}

		.pe-modal__title {
			margin: 0 0 var(--pe-space-4);
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
		}
	}
</style>
