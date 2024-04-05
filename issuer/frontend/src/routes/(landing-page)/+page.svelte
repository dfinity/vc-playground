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
  import { RP_ORIGIN } from '$lib/constants/env-vars';
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import HeroWrapper from '$lib/ui-components/elements/HeroWrapper.svelte';

  onMount(() => {
    setTheme('visitor');
    syncAuth();
  });
</script>

<MainWrapper>
  <HeroWrapper>
    <Stack gap="l">
      <Heading level="1" align="center">Request and Issue Credentials</Heading>
      <Heading level="5" align="center">
        Experience the verifiable credential playground by requesting credentials, issuing
        credentials, or using credentials to view images. You can switch your flow at any time after
        authenticating.
      </Heading>
    </Stack>
    <TwoSlots>
      <HeroCard testId="go-credentials" on:click={() => goto('/credentials')}>
        <User slot="icon" />
        <svelte:fragment slot="title">User</svelte:fragment>
        Obtain credentials from issuers
      </HeroCard>
      <HeroCard on:click={() => goto('/issuer-center')}>
        <Issuer slot="icon" />
        <svelte:fragment slot="title">Issuer</svelte:fragment>
        Create, issue and revoke credentials
      </HeroCard>
    </TwoSlots>
    <Stack gap="md" align="center">
      <Heading level="3" align="center">Want to use credentials?</Heading>
      <a href={RP_ORIGIN} target="_blank">Try it on the Image Sharing Platform</a>
    </Stack>
  </HeroWrapper>
</MainWrapper>
