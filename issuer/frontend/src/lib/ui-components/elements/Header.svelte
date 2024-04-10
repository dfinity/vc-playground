<script lang="ts">
  import { AppBar } from '@skeletonlabs/skeleton';
  import WarningMessage from './WarningMessage.svelte';
  import HeaderTitle from './HeaderTitle.svelte';
  import { page } from '$app/stores';
  import SettingsDropdown from '$lib/components/SettingsDropdown.svelte';

  export let currentRole: 'User' | 'Issuer';
</script>

<AppBar
  slot="header"
  gridColumns="sm:grid-cols-[1fr_3fr_1fr] grid-cols-[1fr_auto]"
  slotTrail="place-self-end"
  slotLead="hidden sm:block"
>
  <WarningMessage slot="lead">This is a demo application</WarningMessage>
  <HeaderTitle>Verifiable Credentials Playground</HeaderTitle>
  <div slot="trail" class="flex gap-4 items-center self-end">
    <div class="hidden sm:flex gap-2">
      <a class={$page.route.id === '/(app)/credentials' ? 'underline' : ''} href="/credentials"
        >Request</a
      >
      <a class={$page.route.id === '/(app)/credentials' ? '' : 'underline'} href="/issuer-center"
        >Issue</a
      >
    </div>
    <SettingsDropdown {currentRole} />
  </div>
</AppBar>
