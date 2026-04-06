<script lang="ts">
	import type { Snippet } from "svelte";
	import AppShell from "$lib/components/layout/AppShell.svelte";
	import { handleAppShellKeydown } from "$lib/handlers/appShellKeydown";

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<svelte:window onkeydown={handleAppShellKeydown} />

<AppShell>
	{@render children()}
</AppShell>
