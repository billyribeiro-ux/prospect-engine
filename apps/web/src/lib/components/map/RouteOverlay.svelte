<script lang="ts">
interface Props {
	visible?: boolean;
}

let { visible = false }: Props = $props();
</script>

<div class="pe-route-overlay" class:pe-route-overlay--on={visible} role="presentation">
	<span class="pe-route-overlay__label">Routes</span>
</div>

<style>
	@layer components {
		.pe-route-overlay {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-route-overlay--on {
			border-color: var(--pe-interactive-primary);
			color: var(--pe-text-primary);
		}
	}
</style>
