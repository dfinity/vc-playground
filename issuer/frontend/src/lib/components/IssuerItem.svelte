<script lang="ts">
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
  import { vcArgumentsValue } from '$lib/utils/vc-arguments-value.utils';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';
  import Badge from '$lib/ui-components/elements/Badge.svelte';
  import {
    countApprovedCredentials,
    countPendingCredentials,
  } from '$lib/utils/count-approved-credentials.utils';
  import { getIssuerDetailStore, type IssuerDetailStore } from '$lib/stores/issuer-detail.store';
  import { authStore } from '$lib/stores/auth.store';

  export let issuer: PublicGroupData;
  export let onClick: (() => void) | undefined = undefined;
  export let isAdmin: boolean = false;

  // Don't show the value when the IssuerItem is rendered for an admin.
  // This might happen because admins can request credentials to themselves.
  let vcArgumentValue: string | number | undefined;
  $: vcArgumentValue = isAdmin ? undefined : vcArgumentsValue(issuer.vc_arguments);

  let issuerStore: IssuerDetailStore;
  $: issuerStore = getIssuerDetailStore({
    identity: $authStore.identity,
    issuerName: issuer.group_name ?? '',
  });
  let approvedCredentials: number;
  $: approvedCredentials = countApprovedCredentials($issuerStore?.members ?? []);
  let pendingCredentials: number;
  $: pendingCredentials = countPendingCredentials($issuerStore?.members ?? []);
</script>

<ListItem testId={`credentials ${issuer.issuer_nickname} ${issuer.group_name}`} {onClick}>
  <svelte:fragment slot="main">
    {issuer.group_name}
    {#if vcArgumentValue !== undefined}
      {` - ${vcArgumentValue}`}
    {/if}
    {#if isAdmin}
      <div>
        <Badge variant="success">{`🪪 Approved: ${approvedCredentials}`}</Badge>
        <Badge variant="default">{`📤 Pending: ${pendingCredentials}`}</Badge>
      </div>
    {/if}
  </svelte:fragment>
  <span slot="sub">{`Issued by ${issuer.issuer_nickname}`}</span>
  <slot name="end" slot="end" />
</ListItem>
