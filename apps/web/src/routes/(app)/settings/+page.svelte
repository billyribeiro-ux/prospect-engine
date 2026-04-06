<script lang="ts">
import ThemeControls from "$lib/components/layout/ThemeControls.svelte";
import ThemeCustomizer from "$lib/components/layout/ThemeCustomizer.svelte";
import SmtpSettingsSection from "$lib/components/settings/SmtpSettingsSection.svelte";
import { messages } from "$lib/i18n/messages/en";
</script>

<div class="workspace-page">
	<h1 class="workspace-page__title">{messages.app.shell.nav.settings}</h1>
	<p class="workspace-page__body">{messages.app.settings.intro}</p>

	<section class="settings-panel" aria-labelledby="settings-appearance-heading">
		<h2 id="settings-appearance-heading" class="settings-panel__title">
			{messages.app.settings.appearanceHeading}
		</h2>
		<div class="settings-panel__surface">
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
	</section>

	<section class="settings-panel">
		<div class="settings-panel__surface settings-panel__surface--wide">
			<ThemeCustomizer labels={messages.app.settings.customColors} />
		</div>
	</section>

	<section class="settings-panel">
		<div class="settings-panel__surface settings-panel__surface--wide">
			<SmtpSettingsSection labels={messages.app.settings.smtp} />
		</div>
	</section>
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
			max-inline-size: 40rem;
			font-size: var(--pe-density-font-size);
			color: var(--pe-text-secondary);
		}

		.settings-panel {
			display: flex;
			flex-direction: column;
			gap: var(--pe-space-3);
		}

		.settings-panel__title {
			margin: 0;
			font-size: var(--pe-font-size-lg);
			font-weight: 600;
			color: var(--pe-text-primary);
		}

		.settings-panel__surface {
			padding: var(--pe-space-4);
			border-radius: var(--pe-radius-md);
			border: 1px solid var(--pe-border-default);
			background: var(--pe-surface-raised);
			max-inline-size: 42rem;
		}

		.settings-panel__surface--wide {
			max-inline-size: min(52rem, 100%);
		}
	}
</style>
