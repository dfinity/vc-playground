<script lang="ts">
  import { login } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import { nonNullish } from '$lib/utils/non-nullish';
  import Button from './Button.svelte';
  import Dropdown from './Dropdown.svelte';
</script>

{#if nonNullish($authStore.identity)}
  <Dropdown />
{:else}
  <Button variant="secondary" on:click={login} loading={$authStore.identity === undefined}
    >Login</Button
  >
{/if}
