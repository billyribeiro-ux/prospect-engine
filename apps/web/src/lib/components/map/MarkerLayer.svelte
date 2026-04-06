<script lang="ts">
interface Props {
	count?: number;
}

let { count = 0 }: Props = $props();
</script>

<div class="pe-marker-layer" role="group" aria-label={`${count} markers`}>
	<span class="pe-marker-layer__label">Markers: {count}</span>
</div>

<style>
	@layer components {
		.pe-marker-layer {
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}
	}
</style>
