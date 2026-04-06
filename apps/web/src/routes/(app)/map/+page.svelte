<script lang="ts">
	import { messages } from "$lib/i18n/messages/en";
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.map}</h1>
	<p class="workspace-page__body">MapLibre map views will appear here.</p>
</div>

<style>
	@layer components {
		.workspace-page__title {
			margin: 0;
			font-size: var(--pe-font-size-2xl);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.workspace-page__body {
			margin-block-start: var(--pe-space-3);
			margin-block-end: 0;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}
	}
</style>
