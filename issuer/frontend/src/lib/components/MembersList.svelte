<script lang="ts">
	import ArticleWrapper from '$lib/ui-components/elements/ArticleWrapper.svelte';
	import Button from '$lib/ui-components/elements/Button.svelte';
	import List from '$lib/ui-components/elements/List.svelte';
	import ListItem from '$lib/ui-components/elements/ListItem.svelte';
</script>

<ArticleWrapper>
	<svelte:fragment slot="title">Members in your group</svelte:fragment>
	<List>
		<ListItem>
			<svelte:fragment slot="main">Name 1</svelte:fragment>
			<svelte:fragment slot="end">
				<Button variant="success">Approve</Button>
				<Button variant="error">Decline</Button>
			</svelte:fragment>
		</ListItem>
		<ListItem>
			<svelte:fragment slot="main">Name 2</svelte:fragment>
			<Button variant="tertiary" slot="end">Revoke</Button>
		</ListItem>
	</List>
</ArticleWrapper>
