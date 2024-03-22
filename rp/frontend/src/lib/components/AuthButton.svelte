<script lang="ts">
  import { login, logout } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
  import Button from './Button.svelte';
</script>

{#if nonNullish($authStore.identity)}
  <Button variant="ghost-secondary" on:click={logout}>Logout</Button>
{:else}
  <Button variant="secondary" on:click={login} loading={$authStore.identity === undefined}
    >Login</Button
  >
{/if}
