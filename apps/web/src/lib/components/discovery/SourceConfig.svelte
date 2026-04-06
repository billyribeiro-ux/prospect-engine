<script lang="ts">
interface Props {
	sourceId?: string;
	label?: string;
}

let { sourceId = "source-1", label = "Source" }: Props = $props();
</script>

<div class="pe-source-config">
	<label class="pe-source-config__label" for={sourceId}>{label}</label>
	<input class="pe-source-config__input" id={sourceId} type="text" placeholder="URL or feed" />
</div>

<style>
	@layer components {
		.pe-source-config {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
		}

		.pe-source-config__label {
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-source-config__input {
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
		}
	}
</style>
