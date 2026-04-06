<script lang="ts">
interface Props {
	visible?: boolean;
}

let { visible = true }: Props = $props();
</script>

<div class="pe-heatmap-layer" class:pe-heatmap-layer--hidden={!visible} role="presentation">
	<span class="pe-heatmap-layer__label">Heatmap</span>
</div>

<style>
	@layer components {
		.pe-heatmap-layer {
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-heatmap-layer--hidden {
			opacity: 0.4;
		}
	}
</style>
