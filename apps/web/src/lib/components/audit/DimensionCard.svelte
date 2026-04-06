<script lang="ts">
interface Props {
	label: string;
	score?: number;
}

let { label, score = 0 }: Props = $props();
</script>

<div class="pe-dimension-card">
	<h3 class="pe-dimension-card__label">{label}</h3>
	<p class="pe-dimension-card__score" aria-label={`Score ${score}`}>{score}</p>
</div>

<style>
	@layer components {
		.pe-dimension-card {
			padding: var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
			font-family: var(--pe-font-family);
		}

		.pe-dimension-card__label {
			margin: 0 0 var(--pe-space-2);
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
			font-weight: 600;
		}

		.pe-dimension-card__score {
			margin: 0;
			font-size: var(--pe-font-size-lg);
			font-weight: 700;
			color: var(--pe-text-primary);
		}
	}
</style>
