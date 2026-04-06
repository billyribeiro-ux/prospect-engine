<script lang="ts">
import type { Snippet } from "svelte";
import Modal from "../shared/Modal.svelte";

interface Props {
	open?: boolean;
	onClose?: () => void;
	children?: Snippet;
}

let { open = false, onClose, children }: Props = $props();
</script>

<Modal open={open} title="Export" {onClose}>
	{#if children}{@render children()}{/if}
</Modal>
