<script lang="ts">
interface Props {
	message: string;
	variant?: "default" | "success" | "error";
}

let { message, variant = "default" }: Props = $props();
</script>

<div
	class="pe-toast"
	class:pe-toast--success={variant === "success"}
	class:pe-toast--error={variant === "error"}
	role="status"
	aria-live="polite"
>
	{message}
</div>

<style>
	@layer components {
		.pe-toast {
			padding: var(--pe-space-3) var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			background: var(--pe-surface-raised);
			color: var(--pe-text-primary);
			border: 1px solid var(--pe-border-default);
		}

		.pe-toast--success {
			border-color: var(--pe-status-success);
		}

		.pe-toast--error {
			border-color: var(--pe-status-error);
		}
	}
</style>
