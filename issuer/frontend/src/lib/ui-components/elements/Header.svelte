<script lang="ts">
  import { AppBar } from '@skeletonlabs/skeleton';
  import WarningMessage from './WarningMessage.svelte';
  import HeaderTitle from './HeaderTitle.svelte';
  import SettingsDropdown from '$lib/components/SettingsDropdown.svelte';

  export let currentRole: 'User' | 'Issuer';
</script>

<AppBar
  slot="header"
  gridColumns="sm:grid-cols-[1fr_1fr_1fr] grid-cols-[1fr_auto]"
  slotTrail="place-self-end"
  slotDefault="hidden sm:flex justify-center"
>
  <HeaderTitle slot="lead">Verifiable Credentials Playground</HeaderTitle>
  <WarningMessage>This is a demo application</WarningMessage>
  <div slot="trail" class="flex gap-4 items-center self-end">
    <div class="hidden sm:flex gap-4">
      <a href="/credentials">View</a>
      <a href="/issuer-center">Publish</a>
    </div>
    <SettingsDropdown {currentRole} />
  </div>
</AppBar>
