<script lang="ts">
import DiscoveryPanel from "$lib/components/discovery/DiscoveryPanel.svelte";
import RadiusSelector from "$lib/components/discovery/RadiusSelector.svelte";
import SourceConfig from "$lib/components/discovery/SourceConfig.svelte";
import { messages } from "$lib/i18n/messages/en";

let radiusMi = $state(25);
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.discover}</h1>
	<p class="workspace-page__body">
		Configure sources and search radius. Full discovery runs will connect to the API in a later phase.
	</p>

	<DiscoveryPanel title="Discovery">
		<div class="workspace-page__stack">
			<SourceConfig label="Primary source" sourceId="pe-discover-source" />
			<RadiusSelector bind:value={radiusMi} unit="mi" />
		</div>
	</DiscoveryPanel>
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
			margin-block-end: var(--pe-space-6);
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.workspace-page__stack {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-5);
			max-inline-size: 28rem;
		}
	}
</style>
