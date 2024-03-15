<script lang="ts">
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth.store';

  $: {
    if ($authStore.identity === null) {
      goto('/');
    }
  }
</script>

{#if $authStore.identity === undefined}
  <slot name="skeleton" />
{:else if $authStore.identity !== null}
  <slot />
{/if}
