<script lang="ts">
  import { goto } from '$app/navigation';
  import { login, syncAuth } from '$lib/services/auth.services';
  import Button from '$lib/ui-components/elements/Button.svelte';
  import { onMount } from 'svelte';
  import '../../app.postcss';
  import { AppShell } from '@skeletonlabs/skeleton';
  import { authStore } from '$lib/stores/auth.store';
  import Hero from '$lib/ui-components/elements/Hero.svelte';
  import HeroTitle from '$lib/ui-components/elements/HeroTitle.svelte';

  const loginUser = async () => {
    await login();
    goto('/home');
  };

  onMount(() => {
    syncAuth();
  });

  $: {
    if ($authStore.identity !== null && $authStore.identity !== undefined) {
      goto('/home');
    }
  }
</script>

<AppShell>
  <Hero>
    <HeroTitle>Welcome to VC Playground</HeroTitle>
    <Button
      loading={$authStore.identity === undefined}
      variant="primary"
      testId="login-button"
      on:click={loginUser}>Connect With Internet Identity</Button
    >
  </Hero>
</AppShell>
