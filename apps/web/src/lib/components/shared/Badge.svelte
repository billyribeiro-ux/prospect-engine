<script lang="ts">
import UiBadge from "@pe/ui/Badge.svelte";
import type { ComponentProps } from "svelte";

let props: ComponentProps<typeof UiBadge> = $props();
</script>

<UiBadge {...props} />
