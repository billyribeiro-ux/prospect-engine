<script lang="ts">
interface Column {
	readonly key: string;
	readonly label: string;
}

interface Props {
	columns: readonly Column[];
	rows: readonly Record<string, unknown>[];
}

let { columns, rows }: Props = $props();
</script>

<div class="pe-data-grid-wrap">
	<table class="pe-data-grid">
		<thead>
			<tr>
				{#each columns as col (col.key)}
					<th scope="col">{col.label}</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each rows as row, ri (ri)}
				<tr>
					{#each columns as col (col.key)}
						<td>{String(row[col.key] ?? "")}</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	@layer components {
		.pe-data-grid-wrap {
			overflow: auto;
			border: 1px solid var(--pe-border-default);
			border-radius: var(--pe-radius-md);
		}

		.pe-data-grid {
			width: 100%;
			border-collapse: collapse;
			font-family: var(--pe-font-family);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-primary);
		}

		.pe-data-grid th,
		.pe-data-grid td {
			padding: var(--pe-space-2) var(--pe-space-3);
			text-align: start;
			border-bottom: 1px solid var(--pe-border-subtle);
		}

		.pe-data-grid th {
			background: var(--pe-surface-sunken);
			font-weight: 600;
		}
	}
</style>
