<script lang="ts">
import "../app.css";
import type { Snippet } from "svelte";
import { browser } from "$app/environment";
import ThemeControls from "$lib/components/layout/ThemeControls.svelte";
import { messages } from "$lib/i18n/messages/en";
import { themeState } from "$lib/stores/theme.svelte";
import type { LayoutData } from "./$types";

interface Props {
	data: LayoutData;
	children: Snippet;
}

let { data, children }: Props = $props();

$effect(() => {
	if (!browser) {
		return;
	}
	themeState.hydrateFromServer(data.theme, data.density);
});
</script>

<a class="pe-skip-link" href="#main">{messages.app.skipToContent}</a>

<div class="app-shell">
	<header class="app-shell__header">
		<div class="app-shell__brand">
			<span class="app-shell__title">{messages.app.productName}</span>
		</div>
		<div class="app-shell__controls">
			<ThemeControls
				labels={{
					themeSection: messages.app.theme.sectionLabel,
					densitySection: messages.app.density.sectionLabel,
					midnight: messages.app.theme.midnight,
					dawn: messages.app.theme.dawn,
					terminal: messages.app.theme.terminal,
					oled: messages.app.theme.oled,
					compact: messages.app.density.compact,
					comfortable: messages.app.density.comfortable,
					spacious: messages.app.density.spacious,
				}}
			/>
		</div>
	</header>

	<main id="main" class="app-shell__main">
		{@render children()}
	</main>
</div>

<style>
	@layer components {
		.app-shell {
			min-block-size: 100dvb;
			display: flex;
			flex-direction: column;
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
		}

		.app-shell__header {
			display: flex;
			flex-wrap: wrap;
			align-items: flex-start;
			justify-content: space-between;
			gap: var(--pe-space-4);
			padding-inline: var(--pe-space-6);
			padding-block: var(--pe-space-4);
			border-block-end: 1px solid var(--pe-border-subtle);
			background: var(--pe-surface-raised);
		}

		.app-shell__brand {
			display: flex;
			align-items: center;
			min-block-size: var(--pe-shell-header-block-size);
		}

		.app-shell__title {
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.app-shell__controls {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-2);
			inline-size: min(100%, 42rem);
		}

		.app-shell__main {
			flex: 1;
			padding-inline: var(--pe-space-6);
			padding-block: var(--pe-space-8);
		}
	}
</style>
