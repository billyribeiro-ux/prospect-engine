<script lang="ts">
import Icon from "@iconify/svelte";
import type { DensityMode } from "@pe/types/theme";
import type { ThemeId } from "@pe/types/theme";
import { themeState } from "$lib/stores/theme.svelte";

interface Props {
	readonly labels: {
		readonly themeSection: string;
		readonly densitySection: string;
		readonly midnight: string;
		readonly dawn: string;
		readonly terminal: string;
		readonly oled: string;
		readonly compact: string;
		readonly comfortable: string;
		readonly spacious: string;
	};
}

const { labels }: Props = $props();

const themeOptions = $derived<
	readonly { readonly id: ThemeId; readonly icon: string; readonly label: string }[]
>([
	{ id: "dawn", icon: "ph:sun", label: labels.dawn },
	{ id: "midnight", icon: "ph:moon", label: labels.midnight },
	{ id: "terminal", icon: "ph:terminal-window", label: labels.terminal },
	{ id: "oled", icon: "ph:circle-half", label: labels.oled },
]);

const densityOptions = $derived<readonly { readonly id: DensityMode; readonly label: string }[]>([
	{ id: "compact", label: labels.compact },
	{ id: "comfortable", label: labels.comfortable },
	{ id: "spacious", label: labels.spacious },
]);

async function onThemeSelect(id: ThemeId): Promise<void> {
	await themeState.setTheme(id);
}

async function onDensitySelect(id: DensityMode): Promise<void> {
	await themeState.setDensity(id);
}
</script>

<section class="theme-controls" aria-label={labels.themeSection}>
	<h2 class="theme-controls__heading">{labels.themeSection}</h2>
	<div class="theme-controls__group" role="group">
		{#each themeOptions as option (option.id)}
			<button
				type="button"
				class="theme-controls__chip"
				class:theme-controls__chip--active={themeState.theme === option.id}
				aria-pressed={themeState.theme === option.id}
				aria-label={option.label}
				onclick={() => {
					void onThemeSelect(option.id);
				}}
			>
				<span class="theme-controls__icon" aria-hidden="true">
					<Icon icon={option.icon} />
				</span>
				<span class="theme-controls__label">{option.label}</span>
			</button>
		{/each}
	</div>
</section>

<section class="theme-controls" aria-label={labels.densitySection}>
	<h2 class="theme-controls__heading">{labels.densitySection}</h2>
	<div class="theme-controls__group" role="group">
		{#each densityOptions as option (option.id)}
			<button
				type="button"
				class="theme-controls__chip"
				class:theme-controls__chip--active={themeState.density === option.id}
				aria-pressed={themeState.density === option.id}
				onclick={() => {
					void onDensitySelect(option.id);
				}}
			>
				<span class="theme-controls__label">{option.label}</span>
			</button>
		{/each}
	</div>
</section>

<style>
	@layer components {
		.theme-controls {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
			padding-block: var(--pe-space-2);
		}

		.theme-controls__heading {
			margin: 0;
			font-size: var(--pe-font-size-sm);
			font-weight: 600;
			color: var(--pe-text-secondary);
		}

		.theme-controls__group {
			display: flex;
			flex-wrap: wrap;
			gap: var(--pe-density-gap);
			align-items: center;
		}

		.theme-controls__chip {
			--_bg: var(--pe-surface-raised);
			--_fg: var(--pe-text-primary);
			--_border: var(--pe-border-default);

			display: inline-flex;
			align-items: center;
			justify-content: center;
			gap: var(--pe-space-2);
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-3);
			padding-block: var(--pe-space-1);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--_border);
			background: var(--_bg);
			color: var(--_fg);
			font-size: var(--pe-density-font-size);
			cursor: pointer;
			transition:
				background var(--pe-duration-fast) var(--pe-easing-default),
				border-color var(--pe-duration-fast) var(--pe-easing-default);
		}

		.theme-controls__chip:hover {
			--_bg: var(--pe-surface-sunken);
		}

		.theme-controls__chip:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.theme-controls__chip--active {
			--_bg: color-mix(in oklch, var(--pe-interactive-primary) 18%, var(--pe-surface-raised));
			--_border: var(--pe-interactive-primary);
		}

		.theme-controls__icon {
			display: inline-flex;
			inline-size: var(--pe-density-icon-size);
			block-size: var(--pe-density-icon-size);
		}

		.theme-controls__icon :global(svg) {
			inline-size: 100%;
			block-size: 100%;
		}

		.theme-controls__label {
			line-height: 1.2;
		}
	}
</style>
