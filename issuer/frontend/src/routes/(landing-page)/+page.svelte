<script lang="ts">
  import { goto } from '$app/navigation';
  import { login, syncAuth } from '$lib/services/auth.services';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { authStore } from '$lib/stores/auth.store';
  import Heading from '$lib/ui-components/elements/Heading.svelte';
  import User from '$lib/ui-components/icons/User.svelte';
  import HeroCard from '$lib/ui-components/elements/HeroCard.svelte';
  import Issuer from '$lib/ui-components/icons/Issuer.svelte';
  import Stack from '$lib/ui-components/elements/Stack.svelte';
  import TwoSlots from '$lib/ui-components/elements/TwoSlots.svelte';
  import { setTheme } from '$lib/services/set-theme';

  const loginUser = (route: string) => async () => {
    await login();
    goto(route);
  };

  onMount(() => {
    setTheme('visitor');
    syncAuth();
  });

  $: {
    if ($authStore.identity !== null && $authStore.identity !== undefined) {
      goto('/credentials');
    }
  }
</script>

<Stack gap="l">
  <Stack gap="l">
    <Heading level="1" align="center">Select your role</Heading>
    <Heading level="5" align="center">
      Experience the playground as an end-user or an issuer. You can switch your role at any time
      after authenticating.
    </Heading>
  </Stack>
  <TwoSlots>
    <HeroCard testId="login-button" on:click={loginUser('/credentials')}>
      <User slot="icon" />
      <svelte:fragment slot="title">User</svelte:fragment>
      Obtain credentials from issuers
    </HeroCard>
    <HeroCard on:click={loginUser('/issuer-center')}>
      <Issuer slot="icon" />
      <svelte:fragment slot="title">Issuer</svelte:fragment>
      Create and issue credentials to users
    </HeroCard>
  </TwoSlots>
</Stack>
