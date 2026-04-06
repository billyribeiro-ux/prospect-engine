<script lang="ts">
interface Metric {
	readonly id: string;
	readonly label: string;
	readonly value: string;
}

interface Props {
	metrics?: readonly Metric[];
}

let { metrics = [] }: Props = $props();
</script>

<div class="pe-tracking-dash" role="region" aria-label="Email tracking">
	<ul class="pe-tracking-dash__list">
		{#each metrics as m (m.id)}
			<li class="pe-tracking-dash__item">
				<span class="pe-tracking-dash__label">{m.label}</span>
				<span class="pe-tracking-dash__value">{m.value}</span>
			</li>
		{/each}
	</ul>
</div>

<style>
	@layer components {
		.pe-tracking-dash {
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-tracking-dash__list {
			margin: 0;
			padding: 0;
			list-style: none;
			display: grid;
			gap: var(--pe-space-2);
		}

		.pe-tracking-dash__item {
			display: flex;
			justify-content: space-between;
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
		}

		.pe-tracking-dash__label {
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}

		.pe-tracking-dash__value {
			font-variant-numeric: tabular-nums;
			font-weight: 600;
		}
	}
</style>
