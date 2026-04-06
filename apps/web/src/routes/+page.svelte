<script lang="ts">
import { messages } from "$lib/i18n/messages/en";
</script>

<section class="home-hero" aria-labelledby="home-title">
	<h1 id="home-title" class="home-hero__title">{messages.app.home.title}</h1>
	<p class="home-hero__tagline">{messages.app.home.tagline}</p>
</section>

<style>
	@layer components {
		.home-hero {
			max-inline-size: 48rem;
		}

		.home-hero__title {
			margin: 0;
			font-size: var(--pe-font-size-3xl);
			font-weight: 700;
			color: var(--pe-text-primary);
		}

		.home-hero__tagline {
			margin-block-start: var(--pe-space-4);
			margin-block-end: 0;
			font-size: var(--pe-font-size-md);
			color: var(--pe-text-secondary);
		}
	}
</style>
