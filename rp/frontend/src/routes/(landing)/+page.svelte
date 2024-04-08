<script lang="ts">
  import HeroCard from '$lib/components/HeroCard.svelte';
  import { goto } from '$app/navigation';
  import MainWrapper from '$lib/components/MainWrapper.svelte';
  import PublishIcon from '$lib/components/PublishIcon.svelte';
  import ViewIcon from '$lib/components/ViewIcon.svelte';
  import { AppShell } from '@skeletonlabs/skeleton';
</script>

<AppShell>
  <MainWrapper>
    <div class="flex flex-col sm:pt-36 gap-6">
      <div class="flex flex-col gap-6">
        <h1 class="h1 text-center">Image Sharing Platform</h1>
        <h3 class="h3 text-center">
          Publish images for people with specific credentials to view, or view gates images using
          your credentials.
        </h3>
      </div>
      <section class="sm:grid sm:grid-cols-2 gap-6 flex flex-col">
        <HeroCard testId="login-button" on:click={() => goto('/share')}>
          <PublishIcon slot="icon" />
          <svelte:fragment slot="title">Publish</svelte:fragment>
          Share images with credential holders
        </HeroCard>
        <HeroCard on:click={() => goto('/feed')}>
          <ViewIcon slot="icon" />
          <svelte:fragment slot="title">View</svelte:fragment>
          View gated images using your credentials
        </HeroCard>
      </section>
    </div>
  </MainWrapper>
</AppShell>
