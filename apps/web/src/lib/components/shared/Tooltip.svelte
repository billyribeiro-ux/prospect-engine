<script lang="ts">
import UiTooltip from "@pe/ui/Tooltip.svelte";
import type { ComponentProps } from "svelte";

let props: ComponentProps<typeof UiTooltip> = $props();
</script>

<UiTooltip {...props} />
