<script lang="ts">
interface Row {
	readonly name: string;
	readonly score: number;
}

interface Props {
	rows?: readonly Row[];
}

let { rows = [] }: Props = $props();
</script>

<div class="pe-competitor-bench">
	<h3 class="pe-competitor-bench__title">Competitors</h3>
	<ul class="pe-competitor-bench__list">
		{#each rows as r (r.name)}
			<li class="pe-competitor-bench__item">
				<span class="pe-competitor-bench__name">{r.name}</span>
				<span class="pe-competitor-bench__score">{r.score}</span>
			</li>
		{/each}
	</ul>
</div>

<style>
	@layer components {
		.pe-competitor-bench {
			font-family: var(--pe-font-family);
			color: var(--pe-text-primary);
		}

		.pe-competitor-bench__title {
			margin: 0 0 var(--pe-space-3);
			font-size: var(--pe-font-size-md);
			font-weight: 600;
		}

		.pe-competitor-bench__list {
			margin: 0;
			padding: 0;
			list-style: none;
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
		}

		.pe-competitor-bench__item {
			display: flex;
			justify-content: space-between;
			padding: var(--pe-space-2) var(--pe-space-3);
			border-radius: var(--pe-radius-sm);
			border: 1px solid var(--pe-border-subtle);
		}

		.pe-competitor-bench__score {
			font-variant-numeric: tabular-nums;
			font-weight: 600;
		}
	}
</style>
