<script lang="ts">
	import Icon from "@iconify/svelte";
	import type { Snippet } from "svelte";
	import CommandPalette from "$lib/components/layout/CommandPalette.svelte";
	import PaneDivider from "$lib/components/layout/PaneDivider.svelte";
	import Sidebar from "$lib/components/layout/Sidebar.svelte";
	import TabBar from "$lib/components/layout/TabBar.svelte";
	import { messages } from "$lib/i18n/messages/en";
	import { appState } from "$lib/stores/app.svelte";
	import { layoutState } from "$lib/stores/layout.svelte";

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	const shell = messages.app.shell;

	const secondaryFlex = $derived(100 - layoutState.primaryPaneSplitPercent);
</script>

<div class="app-shell">
	<header class="app-shell__chrome">
		<div class="app-shell__brand">
			<span class="app-shell__title">{messages.app.productName}</span>
		</div>
		<div class="app-shell__toolbar">
			<button
				type="button"
				class="app-shell__icon-button"
				aria-expanded={!appState.sidebarCollapsed}
				aria-label={shell.sidebar.toggle}
				onclick={() => {
					appState.toggleSidebar();
				}}
			>
				<span class="app-shell__icon-button-icon" aria-hidden="true">
					<Icon icon="ph:list" />
				</span>
			</button>
			<button
				type="button"
				class="app-shell__icon-button"
				aria-expanded={layoutState.secondaryPaneVisible}
				aria-label={shell.toolbar.toggleSplit}
				onclick={() => {
					layoutState.toggleSecondaryPane();
				}}
			>
				<span class="app-shell__icon-button-icon" aria-hidden="true">
					<Icon icon="ph:columns" />
				</span>
			</button>
		</div>
	</header>

	<div class="app-shell__body">
		<Sidebar navLabels={shell.nav} landmarkLabel={shell.sidebar.navLabel} />
		<div class="app-shell__main">
			<TabBar navLabels={shell.nav} landmarkLabel={shell.tabBar.label} />
			<div
				class="app-shell__panes"
				class:app-shell__panes--split={layoutState.secondaryPaneVisible}
			>
				<section
					id="main"
					class="app-shell__primary"
					aria-label={shell.primaryPane.ariaLabel}
					style="flex: {String(layoutState.primaryPaneSplitPercent)} 1 0; min-inline-size: 0;"
				>
					{@render children()}
				</section>
				{#if layoutState.secondaryPaneVisible}
					<PaneDivider ariaLabel={shell.toolbar.toggleSplit} />
					<aside
						class="app-shell__secondary"
						aria-label={shell.secondaryPane.title}
						style="flex: {String(secondaryFlex)} 1 0; min-inline-size: 0;"
					>
						<p class="app-shell__secondary-placeholder">{shell.secondaryPane.placeholder}</p>
					</aside>
				{/if}
			</div>
		</div>
	</div>

	<CommandPalette
		navLabels={shell.nav}
		title={shell.commandPalette.title}
		searchPlaceholder={shell.commandPalette.searchPlaceholder}
		emptyLabel={shell.commandPalette.empty}
		closeLabel={shell.commandPalette.close}
	/>
</div>

<style>
	@layer components {
		.app-shell {
			display: flex;
			flex-direction: column;
			min-block-size: 100dvb;
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
		}

		.app-shell__chrome {
			display: flex;
			flex-wrap: wrap;
			align-items: center;
			justify-content: space-between;
			gap: var(--pe-space-4);
			padding-inline: var(--pe-space-4);
			padding-block: var(--pe-space-3);
			border-block-end: 1px solid var(--pe-border-subtle);
			background: var(--pe-surface-raised);
			min-block-size: var(--pe-shell-header-block-size);
		}

		.app-shell__brand {
			display: flex;
			align-items: center;
		}

		.app-shell__title {
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
		}

		.app-shell__toolbar {
			display: flex;
			align-items: center;
			gap: var(--pe-space-2);
		}

		.app-shell__icon-button {
			display: inline-flex;
			align-items: center;
			justify-content: center;
			inline-size: var(--pe-density-row-height);
			block-size: var(--pe-density-row-height);
			padding: 0;
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-overlay);
			color: var(--pe-text-primary);
			cursor: pointer;
			transition: background var(--pe-duration-fast) var(--pe-easing-default);
		}

		.app-shell__icon-button:hover {
			background: var(--pe-surface-sunken);
		}

		.app-shell__icon-button:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.app-shell__icon-button-icon {
			display: inline-flex;
			inline-size: var(--pe-density-icon-size);
			block-size: var(--pe-density-icon-size);
		}

		.app-shell__icon-button-icon :global(svg) {
			inline-size: 100%;
			block-size: 100%;
		}

		.app-shell__body {
			display: flex;
			flex: 1;
			min-block-size: 0;
		}

		.app-shell__main {
			flex: 1;
			display: flex;
			flex-direction: column;
			min-inline-size: 0;
			padding-inline: var(--pe-space-6);
			padding-block: var(--pe-space-4);
		}

		.app-shell__panes {
			display: flex;
			flex-direction: row;
			flex: 1;
			min-block-size: 0;
			gap: 0;
		}

		.app-shell__panes--split {
			align-items: stretch;
		}

		.app-shell__primary {
			display: flex;
			flex-direction: column;
			min-block-size: 0;
		}

		.app-shell__secondary {
			display: flex;
			flex-direction: column;
			border-inline-start: 1px solid var(--pe-border-subtle);
			background: var(--pe-surface-raised);
			padding: var(--pe-space-4);
		}

		.app-shell__secondary-placeholder {
			margin: 0;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}
	}
</style>
