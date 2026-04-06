<script lang="ts">
import ThemeControls from "$lib/components/layout/ThemeControls.svelte";
import { messages } from "$lib/i18n/messages/en";
</script>

<main id="main" class="marketing-hero" aria-labelledby="marketing-title">
	<h1 id="marketing-title" class="marketing-hero__title">{messages.app.home.title}</h1>
	<p class="marketing-hero__tagline">{messages.app.home.tagline}</p>

	<div class="marketing-hero__theme">
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

	<p class="marketing-hero__cta">
		<a class="marketing-hero__link" href="/discover">Open workspace</a>
	</p>
</main>

<style>
	@layer components {
		.marketing-hero {
			max-inline-size: 48rem;
			margin-inline: auto;
			padding-inline: var(--pe-space-6);
			padding-block: var(--pe-space-12);
		}

		.marketing-hero__title {
			margin: 0;
			font-size: var(--pe-font-size-3xl);
			font-weight: 700;
			color: var(--pe-text-primary);
		}

		.marketing-hero__tagline {
			margin-block-start: var(--pe-space-4);
			margin-block-end: 0;
			font-size: var(--pe-font-size-md);
			color: var(--pe-text-secondary);
		}

		.marketing-hero__theme {
			margin-block-start: var(--pe-space-8);
		}

		.marketing-hero__cta {
			margin-block-start: var(--pe-space-8);
		}

		.marketing-hero__link {
			display: inline-flex;
			align-items: center;
			justify-content: center;
			min-block-size: var(--pe-density-row-height);
			padding-inline: var(--pe-space-4);
			padding-block: var(--pe-space-2);
			border-radius: var(--pe-radius-md);
			background: var(--pe-interactive-primary);
			color: var(--pe-text-inverse);
			font-size: var(--pe-font-size-md);
			font-weight: 600;
			text-decoration: none;
			transition: background var(--pe-duration-fast) var(--pe-easing-default);
		}

		.marketing-hero__link:hover {
			background: var(--pe-interactive-hover);
		}

		.marketing-hero__link:focus-visible {
			outline: var(--pe-focus-ring-width) solid var(--pe-interactive-primary);
			outline-offset: var(--pe-focus-ring-offset);
		}
	}
</style>
