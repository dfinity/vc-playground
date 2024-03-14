<script>
	import AvatarSize from '$lib/ui-components/elements/AvatarSize.svelte';
	import Badge from '$lib/ui-components/elements/Badge.svelte';
	import Button from '$lib/ui-components/elements/Button.svelte';
	import List from '$lib/ui-components/elements/List.svelte';
	import ListItem from '$lib/ui-components/elements/ListItem.svelte';
</script>

<List>
	<ListItem>
		<AvatarSize num={11} slot="start" />
		<svelte:fragment slot="main">Group A</svelte:fragment>
		<Badge slot="end" variant="success">Member</Badge>
	</ListItem>
	<ListItem>
		<AvatarSize num={4321} slot="start" />
		<svelte:fragment slot="main">Group C</svelte:fragment>
		<Button slot="end" variant="primary" size="sm">Join</Button>
	</ListItem>
	<ListItem>
		<AvatarSize num={4} slot="start" />
		<svelte:fragment slot="main">Group B</svelte:fragment>
		<Badge slot="end" variant="default">Pending</Badge>
	</ListItem>
	<ListItem>
		<AvatarSize num={213} slot="start" />
		<svelte:fragment slot="main">Group C</svelte:fragment>
		<Button slot="end" variant="primary" size="sm">Join</Button>
	</ListItem>
</List>
