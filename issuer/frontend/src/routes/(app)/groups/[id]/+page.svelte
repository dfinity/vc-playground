<script lang="ts">
  import MembersList from '$lib/components/MembersList.svelte';
  import Callout from '$lib/ui-components/elements/Callout.svelte';
  import DefaultPage from '$lib/ui-components/page-layouts/DefaultPage.svelte';
</script>

<DefaultPage>
  <svelte:fragment slot="title">My Fav Group</svelte:fragment>
  <Callout>
    <p>🎉 You are the owner of this group.</p>
  </Callout>
  <MembersList />
</DefaultPage>
