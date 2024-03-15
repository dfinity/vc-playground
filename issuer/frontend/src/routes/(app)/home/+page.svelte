<script lang="ts">
  import GroupsList from '$lib/components/GroupsList.svelte';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import FooterActionsWrapper from '$lib/ui-components/elements/FooterActionsWrapper.svelte';
  import { Tab, TabGroup, localStorageStore } from '@skeletonlabs/skeleton';
  import type { PublicGroupData } from '../../../declarations/meta_issuer.did';
  import type { Writable } from 'svelte/store';
  import AuthGuard from '$lib/components/AuthGuard.svelte';

  // Persist the selected tab in the local storage.
  const tabStore: Writable<number> = localStorageStore('groupsTab', 0);
  let tabSet = $tabStore;
  $: tabStore.set(tabSet);

  const noMyGroupsMessage =
    'Create a group to issue Verifiable Credentials that grant people access to funny images on the Relying Party app.';

  // TODO: Replace with data from the backend.
  const yesterday = new Date(new Date().getTime() - 24 * 60 * 60 * 1000);
  const groups: PublicGroupData[] = [
    {
      membership_status: [{ Accepted: null }],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 32,
      },
      group_name: 'Group A',
    },
    {
      membership_status: [],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 705,
      },
      group_name: 'Group B',
    },
    {
      membership_status: [{ PendingReview: null }],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 0,
      },
      group_name: 'Group C',
    },
    {
      membership_status: [{ Rejected: null }],
      is_owner: [false],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 2100,
      },
      group_name: 'Group D',
    },
    {
      membership_status: [{ Accepted: null }],
      is_owner: [true],
      stats: {
        created_timestamp_ns: BigInt(yesterday.getTime()) * 1000000n,
        member_count: 11,
      },
      group_name: 'Group Z',
    },
  ];
</script>

<TabGroup justify="justify-center">
  <Tab bind:group={tabSet} name="all-groups" value={0}>All Groups</Tab>
  <Tab bind:group={tabSet} name="all-groups" value={1}>My memberships</Tab>
  <Tab bind:group={tabSet} name="my-groups" value={2}>My Groups</Tab>
  <!-- Tab Panels --->
  <svelte:fragment slot="panel">
    <AuthGuard>
      {#if tabSet === 0}
        <GroupsList {groups} />
      {:else if tabSet === 1}
        <GroupsList
          groups={groups.filter(
            ({ membership_status, is_owner }) =>
              membership_status[0] !== undefined &&
              ('Accepted' in membership_status[0] || 'PendingReview' in membership_status[0]) &&
              !is_owner[0]
          )}
        />
      {:else if tabSet === 2}
        <FooterActionsWrapper>
          <GroupsList
            groups={groups.filter(({ is_owner }) => is_owner[0])}
            noGroupsMessage={noMyGroupsMessage}
          />
          <Button variant="primary" slot="actions">Create Group</Button>
        </FooterActionsWrapper>
      {/if}
      <svelte:fragment slot="skeleton">
        <GroupsList groups={undefined} />
      </svelte:fragment>
    </AuthGuard>
  </svelte:fragment>
</TabGroup>
