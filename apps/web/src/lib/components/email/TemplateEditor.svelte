<script lang="ts">
interface Props {
	value?: string;
	placeholder?: string;
}

let { value = $bindable(""), placeholder = "Write your template…" }: Props = $props();
</script>

<div class="pe-template-editor">
	<label class="pe-template-editor__label" for="pe-template-body">Template</label>
	<textarea
		id="pe-template-body"
		class="pe-template-editor__area"
		rows="8"
		{placeholder}
		bind:value
	></textarea>
</div>

<style>
	@layer components {
		.pe-template-editor {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-template-editor__label {
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-template-editor__area {
			font-family: var(--pe-font-mono);
			font-size: var(--pe-font-size-sm);
			padding: var(--pe-space-3);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
			resize: vertical;
		}
	}
</style>
