<script lang="ts">
  import { login, logout } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import IconCopy from './IconCopy.svelte';
  import PopoverDropdown from '$lib/ui-components/elements/PopoverDropdown.svelte';
  import NavBarItem from '$lib/ui-components/elements/NavBarItem.svelte';
  import { isNullish } from '$lib/utils/is-nullish.utils';
  import { goto } from '$app/navigation';

  export let currentRole: 'User' | 'Issuer';

  let principal: string;
  $: principal = $authStore.identity?.getPrincipal().toText() ?? '';

  const copyToClipboard = async () => await navigator.clipboard.writeText(principal);

  const switchRole = () => {
    if (currentRole === 'User') {
      goto('/issuer-center');
    } else {
      goto('/credentials');
    }
  };
</script>

<PopoverDropdown>
  <ul>
    <li>
      <NavBarItem on:click={switchRole}
        >{`Go to ${currentRole === 'Issuer' ? 'request' : 'issue'}`}</NavBarItem
      >
    </li>
    {#if isNullish($authStore.identity)}
      <li>
        <NavBarItem on:click={login}>Login</NavBarItem>
      </li>
    {:else}
      <li>
        <NavBarItem on:click={copyToClipboard}>
          <span><IconCopy /></span>
          <span class="truncate">{principal}</span>
        </NavBarItem>
      </li>
      <li>
        <NavBarItem on:click={logout}>Logout</NavBarItem>
      </li>
    {/if}
  </ul>
</PopoverDropdown>
