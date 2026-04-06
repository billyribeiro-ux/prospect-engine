<script lang="ts">
interface Props {
	logoUrl?: string;
	accent?: string;
}

let { logoUrl = $bindable(""), accent = $bindable("#3b82f6") }: Props = $props();
</script>

<div class="pe-white-label">
	<label class="pe-white-label__field" for="pe-wl-logo">Logo URL</label>
	<input id="pe-wl-logo" class="pe-white-label__input" type="url" bind:value={logoUrl} />
	<label class="pe-white-label__field" for="pe-wl-accent">Accent</label>
	<input id="pe-wl-accent" class="pe-white-label__input" type="color" bind:value={accent} />
</div>

<style>
	@layer components {
		.pe-white-label {
			display: grid;
			gap: var(--pe-space-2);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-white-label__field {
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-white-label__input {
			font-size: var(--pe-font-size-sm);
			padding: var(--pe-space-2);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
		}
	}
</style>
