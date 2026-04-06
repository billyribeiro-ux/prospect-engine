<script lang="ts">
	import Iconify from "@iconify/svelte";

	interface Props {
		icon: string;
		"aria-hidden"?: boolean;
	}

	let { icon, "aria-hidden": ariaHidden = true }: Props = $props();
</script>

<span class="pe-icon" aria-hidden={ariaHidden ? "true" : undefined}>
	<Iconify {icon} />
</span>

<style>
	@layer components {
		.pe-icon {
			display: inline-flex;
			inline-size: var(--pe-density-icon-size);
			block-size: var(--pe-density-icon-size);
		}

		.pe-icon :global(svg) {
			inline-size: 100%;
			block-size: 100%;
		}
	}
</style>
