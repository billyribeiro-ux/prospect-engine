<script lang="ts">
	import Icon from "@iconify/svelte";
	import { tick } from "svelte";
	import { APP_NAV_ITEMS } from "$lib/constants/navigation";
	import type { ShellNavLabels } from "$lib/i18n/messages/en";
	import { appState } from "$lib/stores/app.svelte";

	interface Props {
		readonly navLabels: ShellNavLabels;
		readonly title: string;
		readonly searchPlaceholder: string;
		readonly emptyLabel: string;
		readonly closeLabel: string;
	}

	let { navLabels, title, searchPlaceholder, emptyLabel, closeLabel }: Props = $props();

	let query = $state("");
	let queryInput = $state<HTMLInputElement | undefined>(undefined);

	const filteredItems = $derived.by(() => {
		const q = query.trim().toLowerCase();
		if (q.length === 0) {
			return APP_NAV_ITEMS;
		}
		return APP_NAV_ITEMS.filter((item) => {
			const label = navLabels[item.id].toLowerCase();
			return label.includes(q);
		});
	});

	function labelFor(id: (typeof APP_NAV_ITEMS)[number]["id"]): string {
		return navLabels[id];
	}

	$effect(() => {
		if (!appState.commandPaletteOpen) {
			query = "";
			return;
		}
		void tick().then(() => {
			queryInput?.focus();
		});
	});

	function onOverlayClick(): void {
		appState.closeCommandPalette();
	}

	function onOverlayKeydown(event: KeyboardEvent): void {
		if (event.key === "Escape") {
			event.preventDefault();
			appState.closeCommandPalette();
		}
	}
</script>

{#if appState.commandPaletteOpen}
	<div
		class="command-palette"
		role="dialog"
		tabindex="-1"
		aria-modal="true"
		aria-labelledby="command-palette-title"
		aria-describedby="command-palette-desc"
		onkeydown={onOverlayKeydown}
	>
		<button
			type="button"
			class="command-palette__backdrop"
			aria-label={closeLabel}
			onclick={onOverlayClick}
		></button>
		<div class="command-palette__panel">
			<h2 id="command-palette-title" class="command-palette__title">{title}</h2>
			<p id="command-palette-desc" class="pe-visually-hidden">
				{searchPlaceholder}
			</p>
			<div class="command-palette__search">
				<span class="command-palette__search-icon" aria-hidden="true">
					<Icon icon="ph:magnifying-glass" />
				</span>
				<input
					bind:this={queryInput}
					bind:value={query}
					class="command-palette__input"
					type="search"
					autocomplete="off"
					autocorrect="off"
					spellcheck="false"
					placeholder={searchPlaceholder}
					aria-label={searchPlaceholder}
				/>
			</div>
			<ul class="command-palette__list" role="list">
				{#each filteredItems as item (item.id)}
					<li role="listitem">
						<a
							class="command-palette__item"
							href={item.href}
							onclick={() => {
								appState.closeCommandPalette();
							}}
						>
							<span class="command-palette__item-icon" aria-hidden="true">
								<Icon icon={item.icon} />
							</span>
							<span class="command-palette__item-label">{labelFor(item.id)}</span>
						</a>
					</li>
				{/each}
			</ul>
			{#if filteredItems.length === 0}
				<p class="command-palette__empty">{emptyLabel}</p>
			{/if}
		</div>
	</div>
{/if}

<style>
	@layer components {
		.command-palette {
			position: fixed;
			inset: 0;
			z-index: var(--pe-z-command-palette);
			display: flex;
			align-items: flex-start;
			justify-content: center;
			padding-block-start: 12vh;
			padding-inline: var(--pe-space-4);
		}

		.command-palette__backdrop {
			position: absolute;
			inset: 0;
			margin: 0;
			padding: 0;
			border: none;
			background: oklch(0.15 0.02 240 / 0.45);
			cursor: pointer;
		}

		.command-palette__panel {
			position: relative;
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
			inline-size: min(100%, 32rem);
			max-block-size: min(70dvb, 28rem);
			padding: var(--pe-space-4);
			border-radius: var(--pe-radius-lg);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-overlay);
			box-shadow: var(--pe-shadow-xl);
		}

		.command-palette__title {
			margin: 0;
			font-size: var(--pe-font-size-md);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.command-palette__search {
			display: flex;
			align-items: center;
			gap: var(--pe-space-2);
			padding-inline: var(--pe-space-3);
			padding-block: var(--pe-space-2);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
		}

		.command-palette__search-icon {
			display: inline-flex;
			inline-size: var(--pe-density-icon-size);
			block-size: var(--pe-density-icon-size);
			flex-shrink: 0;
			color: var(--pe-text-tertiary);
		}

		.command-palette__search-icon :global(svg) {
			inline-size: 100%;
			block-size: 100%;
		}

		.command-palette__input {
			flex: 1;
			min-inline-size: 0;
			border: none;
			background: transparent;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-primary);
		}

		.command-palette__input:focus {
			outline: none;
		}

		.command-palette__list {
			list-style: none;
			margin: 0;
			padding: 0;
			overflow: auto;
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
		}

		.command-palette__item {
			display: flex;
			align-items: center;
			gap: var(--pe-space-2);
			padding-inline: var(--pe-space-3);
			padding-block: var(--pe-space-2);
			border-radius: var(--pe-radius-md);
			color: var(--pe-text-primary);
			text-decoration: none;
			font-size: var(--pe-density-font-size);
			transition: background var(--pe-duration-fast) var(--pe-easing-default);
		}

		.command-palette__item:hover {
			background: var(--pe-surface-sunken);
		}

		.command-palette__item:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.command-palette__item-icon {
			display: inline-flex;
			inline-size: var(--pe-density-icon-size);
			block-size: var(--pe-density-icon-size);
			flex-shrink: 0;
		}

		.command-palette__item-icon :global(svg) {
			inline-size: 100%;
			block-size: 100%;
		}

		.command-palette__empty {
			margin: 0;
			font-size: var(--pe-font-size-sm);
			color: var(--pe-text-secondary);
		}
	}
</style>
