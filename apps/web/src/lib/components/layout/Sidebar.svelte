<script lang="ts">
import Icon from "@iconify/svelte";
import { page } from "$app/state";
import { APP_NAV_ITEMS } from "$lib/constants/navigation";
import type { ShellNavLabels } from "$lib/i18n/messages/en";
import { appState } from "$lib/stores/app.svelte";

interface Props {
	readonly navLabels: ShellNavLabels;
	readonly landmarkLabel: string;
}

let { navLabels, landmarkLabel }: Props = $props();

const pathname = $derived(page.url.pathname);

function labelFor(id: (typeof APP_NAV_ITEMS)[number]["id"], m: ShellNavLabels): string {
	return m[id];
}
</script>

<nav
	class="app-sidebar"
	class:app-sidebar--collapsed={appState.sidebarCollapsed}
	aria-label={landmarkLabel}
	data-testid="app-sidebar"
>
	<ul class="app-sidebar__list" role="list">
		{#each APP_NAV_ITEMS as item (item.id)}
			<li class="app-sidebar__item" role="listitem">
				<a
					class="app-sidebar__link"
					class:app-sidebar__link--active={pathname === item.href ||
						pathname.startsWith(`${item.href}/`)}
					href={item.href}
				>
					<span class="app-sidebar__icon" aria-hidden="true">
						<Icon icon={item.icon} />
					</span>
					<span class="app-sidebar__text">{labelFor(item.id, navLabels)}</span>
				</a>
			</li>
		{/each}
	</ul>
</nav>

<style>
	@layer components {
		.app-sidebar {
			--_inline: var(--pe-shell-sidebar-inline-size);

			display: flex;
			flex-direction: column;
			flex-shrink: 0;
			inline-size: var(--_inline);
			border-inline-end: 1px solid var(--pe-border-subtle);
			background: var(--pe-surface-raised);
			transition: inline-size var(--pe-duration-normal) var(--pe-easing-default);
		}

		.app-sidebar--collapsed {
			--_inline: var(--pe-shell-sidebar-inline-size-collapsed);
		}

		.app-sidebar--collapsed .app-sidebar__text {
			position: absolute;
			inline-size: 1px;
			block-size: 1px;
			padding: 0;
			margin: -1px;
			overflow: hidden;
			clip: rect(0, 0, 0, 0);
			white-space: nowrap;
			border: 0;
		}

		.app-sidebar__list {
			list-style: none;
			margin: 0;
			padding: var(--pe-space-2);
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-1);
		}

		.app-sidebar__link {
			position: relative;
			display: flex;
			align-items: center;
			gap: var(--pe-space-2);
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-2);
			padding-block: var(--pe-space-1);
			border-radius: var(--pe-radius-md);
			color: var(--pe-text-primary);
			text-decoration: none;
			font-size: var(--pe-density-font-size);
			transition: background var(--pe-duration-fast) var(--pe-easing-default);
		}

		.app-sidebar__link:hover {
			background: var(--pe-surface-sunken);
		}

		.app-sidebar__link:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.app-sidebar__link--active {
			background: color-mix(in oklch, var(--pe-interactive-primary) 16%, var(--pe-surface-raised));
			color: var(--pe-text-primary);
			font-weight: 600;
		}

		.app-sidebar__icon {
			display: inline-flex;
			inline-size: var(--pe-density-icon-size);
			block-size: var(--pe-density-icon-size);
			flex-shrink: 0;
		}

		.app-sidebar__icon :global(svg) {
			inline-size: 100%;
			block-size: 100%;
		}
	}
</style>
