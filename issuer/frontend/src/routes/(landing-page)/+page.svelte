<script lang="ts">
  import { syncAuth } from '$lib/services/auth.services';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import Heading from '$lib/ui-components/elements/Heading.svelte';
  import HeroCard from '$lib/ui-components/elements/HeroCard.svelte';
  import Stack from '$lib/ui-components/elements/Stack.svelte';
  import TwoSlots from '$lib/ui-components/elements/TwoSlots.svelte';
  import { setTheme } from '$lib/services/set-theme';
  import { RP_ORIGIN } from '$lib/constants/env-vars';
  import MainWrapper from '$lib/ui-components/elements/MainWrapper.svelte';
  import HeroWrapper from '$lib/ui-components/elements/HeroWrapper.svelte';
  import IssuerIcon from '$lib/ui-components/icons/IssuerIcon.svelte';
  import UserIcon from '$lib/ui-components/icons/UserIcon.svelte';
  import { AppShell } from '@skeletonlabs/skeleton';
  import Button from '$lib/ui-components/elements/Button.svelte';

  onMount(() => {
    setTheme('visitor');
    syncAuth();
  });
</script>

<AppShell>
  <MainWrapper>
    <HeroWrapper>
      <Stack gap="l">
        <Heading level="1" align="center">Request and Issue Credentials</Heading>
        <Heading level="5" align="center">
          Experience the verifiable credential playground by requesting credentials, issuing
          credentials, or using credentials to view images.
        </Heading>
      </Stack>
      <TwoSlots>
        <HeroCard testId="go-credentials" href="/credentials">
          <UserIcon slot="icon" />
          <svelte:fragment slot="title">Request</svelte:fragment>
          Obtain credentials from issuers
        </HeroCard>
        <HeroCard testId="go-issuer-center" href="/issuer-center">
          <IssuerIcon slot="icon" />
          <svelte:fragment slot="title">Issue</svelte:fragment>
          Create, issue and revoke credentials
        </HeroCard>
      </TwoSlots>
      <Stack gap="md" align="center">
        <Heading level="3" align="center">Want to use credentials?</Heading>
        <Button variant="primary" href={RP_ORIGIN}>Try it on the Image Sharing Platform</Button>
      </Stack>
    </HeroWrapper>
  </MainWrapper>
</AppShell>
