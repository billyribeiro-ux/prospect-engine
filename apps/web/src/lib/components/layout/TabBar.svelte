<script lang="ts">
import { page } from "$app/state";
import { APP_NAV_ITEMS } from "$lib/constants/navigation";
import type { ShellNavLabels } from "$lib/i18n/messages/en";

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

<nav class="app-tab-bar" aria-label={landmarkLabel}>
	{#each APP_NAV_ITEMS as item (item.id)}
		<a
			class="app-tab-bar__tab"
			class:app-tab-bar__tab--active={pathname === item.href ||
				pathname.startsWith(`${item.href}/`)}
			href={item.href}
			aria-current={pathname === item.href || pathname.startsWith(`${item.href}/`)
				? "page"
				: undefined}
		>
			{labelFor(item.id, navLabels)}
		</a>
	{/each}
</nav>

<style>
	@layer components {
		.app-tab-bar {
			display: flex;
			flex-wrap: wrap;
			gap: var(--pe-space-1);
			padding-block-end: var(--pe-space-3);
			border-block-end: 1px solid var(--pe-border-subtle);
			margin-block-end: var(--pe-space-4);
		}

		.app-tab-bar__tab {
			display: inline-flex;
			align-items: center;
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-3);
			padding-block: var(--pe-space-1);
			border-radius: var(--pe-radius-md);
			border: 1px solid transparent;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
			text-decoration: none;
			transition:
				background var(--pe-duration-fast) var(--pe-easing-default),
				color var(--pe-duration-fast) var(--pe-easing-default),
				border-color var(--pe-duration-fast) var(--pe-easing-default);
		}

		.app-tab-bar__tab:hover {
			background: var(--pe-surface-sunken);
			color: var(--pe-text-primary);
		}

		.app-tab-bar__tab:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}

		.app-tab-bar__tab--active {
			border-color: var(--pe-border-default);
			background: var(--pe-surface-overlay);
			color: var(--pe-text-primary);
			font-weight: 600;
		}
	}
</style>
