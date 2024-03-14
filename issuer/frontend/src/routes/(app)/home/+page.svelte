<script lang="ts">
	import GroupsList from "$lib/components/GroupsList.svelte";
import { Tab, TabGroup } from "@skeletonlabs/skeleton";

	let tabSet = 0;
</script>
<TabGroup justify="justify-center">
	<Tab bind:group={tabSet} name="all-groups" value={0}>All Groups</Tab>
	<Tab bind:group={tabSet} name="my-groups" value={1}>My Groups</Tab>
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#if tabSet === 0}
			<GroupsList />
		{:else if tabSet === 1}
			<div>My Groups</div>
		{/if}
	</svelte:fragment>
</TabGroup>
