<script lang="ts">
import "../app.css";
import type { Snippet } from "svelte";
import { browser } from "$app/environment";
import { messages } from "$lib/i18n/messages/en";
import { customThemeState } from "$lib/stores/customTheme.svelte";
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
	customThemeState.applyAll();
});
</script>

<a class="pe-skip-link" href="#main">{messages.app.skipToContent}</a>

<div class="pe-app-root">
	{@render children()}
</div>
