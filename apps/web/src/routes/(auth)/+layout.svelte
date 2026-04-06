<script lang="ts">
	import type { Snippet } from "svelte";

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<div class="auth-layout">
	<div class="auth-layout__panel">
		{@render children()}
	</div>
</div>

<style>
	@layer components {
		.auth-layout {
			min-block-size: 100dvb;
			display: flex;
			align-items: center;
			justify-content: center;
			padding: var(--pe-space-6);
			background: var(--pe-surface-base);
			color: var(--pe-text-primary);
		}

		.auth-layout__panel {
			inline-size: min(100%, 24rem);
			padding: var(--pe-space-8);
			border-radius: var(--pe-radius-lg);
			border: 1px solid var(--pe-border-subtle);
			background: var(--pe-surface-raised);
			box-shadow: var(--pe-shadow-md);
		}
	}
</style>
