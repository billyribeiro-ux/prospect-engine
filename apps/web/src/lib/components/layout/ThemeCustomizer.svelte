<script lang="ts">
import { browser } from "$app/environment";
import { customThemeState } from "$lib/stores/customTheme.svelte";
import { themeState } from "$lib/stores/theme.svelte";
import { type CssVarProbe, getCssVarAsHex } from "$lib/theme/cssVarColor";
import {
	CUSTOM_COLOR_GROUPS,
	type CustomColorGroupId,
	type CustomColorLabels,
} from "$lib/theme/customColorTokens";

interface Props {
	readonly labels: typeof import("$lib/i18n/messages/en")["messages"]["app"]["settings"]["customColors"];
}

const { labels }: Props = $props();

function groupHeading(id: CustomColorGroupId): string {
	return labels[id];
}

function tokenLabel(key: keyof CustomColorLabels): string {
	return labels[key];
}

function hexValue(varName: string, probe: CssVarProbe): string {
	void themeState.theme;
	void themeState.density;
	void customThemeState.overrides;
	const o = customThemeState.overrides[varName];
	if (o) {
		return o;
	}
	if (!browser) {
		return "#888888";
	}
	return getCssVarAsHex(varName, probe);
}

function onPick(varName: string, value: string): void {
	customThemeState.setToken(varName, value);
}

function onClear(varName: string): void {
	customThemeState.clearToken(varName);
}
</script>

<section class="theme-customizer" aria-labelledby="theme-customizer-title">
	<h2 id="theme-customizer-title" class="theme-customizer__title">{labels.sectionTitle}</h2>
	<p class="theme-customizer__lead">{labels.sectionLead}</p>

	<div class="theme-customizer__toolbar">
		<button
			type="button"
			class="theme-customizer__reset-all"
			onclick={() => {
				customThemeState.clearAll();
			}}
		>
			{labels.resetAll}
		</button>
	</div>

	{#each CUSTOM_COLOR_GROUPS as group (group.id)}
		<fieldset class="theme-customizer__group">
			<legend class="theme-customizer__group-title">{groupHeading(group.id)}</legend>
			<div class="theme-customizer__rows">
				{#each group.tokens as token (token.varName)}
					<div class="theme-customizer__row">
						<label class="theme-customizer__label" for="pe-color-{token.labelKey}">
							{tokenLabel(token.labelKey)}
						</label>
						<div class="theme-customizer__controls">
							{#key `${themeState.theme}-${themeState.density}-${hexValue(token.varName, token.probe)}`}
								<input
									id="pe-color-{token.labelKey}"
									class="theme-customizer__input"
									type="color"
									value={hexValue(token.varName, token.probe)}
									aria-label={tokenLabel(token.labelKey)}
									oninput={(e) => {
										onPick(token.varName, e.currentTarget.value);
									}}
								/>
							{/key}
							<button
								type="button"
								class="theme-customizer__clear"
								onclick={() => {
									onClear(token.varName);
								}}
							>
								{labels.useDefault}
							</button>
						</div>
					</div>
				{/each}
			</div>
		</fieldset>
	{/each}
</section>

<style>
	@layer components {
		.theme-customizer {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-4);
			padding-block: var(--pe-space-2);
		}

		.theme-customizer__title {
			margin: 0;
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.theme-customizer__lead {
			margin: 0;
			max-inline-size: 48rem;
			font-size: var(--pe-font-size-sm);
			line-height: 1.5;
			color: var(--pe-text-secondary);
		}

		.theme-customizer__toolbar {
			display: flex;
			flex-wrap: wrap;
			gap: var(--pe-space-2);
		}

		.theme-customizer__reset-all {
			display: inline-flex;
			align-items: center;
			justify-content: center;
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-3);
			padding-block: var(--pe-space-2);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-sunken);
			color: var(--pe-text-primary);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			cursor: pointer;
		}

		.theme-customizer__reset-all:hover {
			background: var(--pe-surface-raised);
		}

		.theme-customizer__reset-all:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.theme-customizer__group {
			margin: 0;
			padding: 0;
			border: none;
			min-inline-size: 0;
		}

		.theme-customizer__group-title {
			padding: 0;
			margin-block-end: var(--pe-space-2);
			font-size: var(--pe-font-size-sm);
			font-weight: 600;
			color: var(--pe-text-secondary);
		}

		.theme-customizer__rows {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
		}

		.theme-customizer__row {
			display: grid;
			grid-template-columns: minmax(8rem, 1fr) auto;
			gap: var(--pe-space-3);
			align-items: center;
		}

		@media (max-width: 36rem) {
			.theme-customizer__row {
				grid-template-columns: 1fr;
			}
		}

		.theme-customizer__label {
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-primary);
		}

		.theme-customizer__controls {
			display: flex;
			flex-wrap: wrap;
			align-items: center;
			gap: var(--pe-space-2);
			justify-content: flex-end;
		}

		.theme-customizer__input {
			inline-size: 3rem;
			block-size: 2rem;
			padding: 0;
			border: 1px solid var(--pe-border-default);
			border-radius: var(--pe-radius-sm);
			background: var(--pe-surface-base);
			cursor: pointer;
		}

		.theme-customizer__input:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.theme-customizer__clear {
			display: inline-flex;
			align-items: center;
			padding-inline: var(--pe-space-2);
			padding-block: var(--pe-space-1);
			border-radius: var(--pe-radius-sm);
			border: 1px solid transparent;
			background: transparent;
			color: var(--pe-interactive-primary);
			font-family: var(--pe-font-family);
			font-size: var(--pe-font-size-sm);
			cursor: pointer;
			text-decoration: underline;
			text-underline-offset: 2px;
		}

		.theme-customizer__clear:hover {
			color: var(--pe-interactive-hover);
		}
	}
</style>
