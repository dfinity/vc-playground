<script lang="ts">
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import { vcArgumentsValue } from '$lib/utils/vc-arguments-value.utils';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';

  export let issuer: PublicGroupData;
  export let onClick: (() => void) | undefined = undefined;
  export let isAdmin: boolean = false;

  // Don't show the value when the IssuerItem is rendered for an admin.
  // This might happen because admins can request credentials to themselves.
  let vcArgumentValue: string | number | undefined;
  $: vcArgumentValue = isAdmin ? undefined : vcArgumentsValue(issuer.vc_arguments);
</script>

<ListItem testId={`credentials ${issuer.issuer_nickname} ${issuer.group_name}`} {onClick}>
  <svelte:fragment slot="main">
    {issuer.group_name}
    {#if vcArgumentValue !== undefined}
      {` - ${vcArgumentValue}`}
    {/if}
  </svelte:fragment>
  <span slot="sub">{`Issued by ${issuer.issuer_nickname}`}</span>
  <slot name="end" slot="end" />
</ListItem>
