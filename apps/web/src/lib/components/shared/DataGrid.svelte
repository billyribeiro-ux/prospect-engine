<script lang="ts">
import DataGrid from "@pe/ui/DataGrid.svelte";
import type { ComponentProps } from "svelte";

let props: ComponentProps<typeof DataGrid> = $props();
</script>

<DataGrid {...props} />
