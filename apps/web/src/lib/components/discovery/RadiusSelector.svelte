<script lang="ts">
interface Props {
	value?: number;
	min?: number;
	max?: number;
	unit?: string;
}

let { value = $bindable(25), min = 1, max = 100, unit = "mi" }: Props = $props();
</script>

<div class="pe-radius-selector">
	<label class="pe-radius-selector__label" for="pe-radius">Radius ({unit})</label>
	<div class="pe-radius-selector__row">
		<input
			id="pe-radius"
			class="pe-radius-selector__range"
			type="range"
			{min}
			{max}
			bind:value
		/>
		<span class="pe-radius-selector__value" aria-live="polite">{value}</span>
	</div>
</div>

<style>
	@layer components {
		.pe-radius-selector {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-radius-selector__label {
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-radius-selector__row {
			display: flex;
			align-items: center;
			gap: var(--pe-space-3);
		}

		.pe-radius-selector__range {
			flex: 1;
		}

		.pe-radius-selector__value {
			min-inline-size: 2ch;
			font-variant-numeric: tabular-nums;
			font-size: var(--pe-font-size-md);
		}
	}
</style>
