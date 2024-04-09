<script lang="ts">
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';

  export let issuer: PublicGroupData;
  export let onClick: (() => void) | undefined = undefined;
</script>

<ListItem testId={`credentials ${issuer.issuer_nickname} ${issuer.group_name}`} {onClick}>
  <svelte:fragment slot="main">{issuer.group_name}</svelte:fragment>
  <span slot="sub">{`Issued by ${issuer.issuer_nickname}`}</span>
  <slot name="end" slot="end" />
</ListItem>
