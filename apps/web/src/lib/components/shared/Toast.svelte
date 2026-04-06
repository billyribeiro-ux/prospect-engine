<script lang="ts">
import UiToast from "@pe/ui/Toast.svelte";
import type { ComponentProps } from "svelte";

let props: ComponentProps<typeof UiToast> = $props();
</script>

<UiToast {...props} />
