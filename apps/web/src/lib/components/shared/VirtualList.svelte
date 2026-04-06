<script lang="ts" generics="T">
import VirtualList from "@pe/ui/VirtualList.svelte";
import type { Snippet } from "svelte";

let {
	items,
	item,
}: {
	items: readonly T[];
	item: Snippet<[T, number]>;
} = $props();
</script>

<VirtualList {items} {item} />
