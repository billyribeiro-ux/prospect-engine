<script lang="ts">
import UiModal from "@pe/ui/Modal.svelte";
import type { ComponentProps } from "svelte";

let props: ComponentProps<typeof UiModal> = $props();
</script>

<UiModal {...props} />
