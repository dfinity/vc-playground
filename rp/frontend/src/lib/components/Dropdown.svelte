<script lang="ts">
  import { login, logout } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import { getToastStore, popup, type PopupSettings } from '@skeletonlabs/skeleton';
  import IconCopy from './IconCopy.svelte';
  import NavBarItem from './NavBarItem.svelte';
  import { goto } from '$app/navigation';
  import { isNullish } from '$lib/utils/is-nullish.utils';

  export let currentRole: 'View' | 'Publish';

  const toastStore = getToastStore();

  const popupCombobox: PopupSettings = {
    event: 'click',
    target: 'popupCombobox',
    placement: 'bottom',
  };

  let principal: string;
  $: principal = $authStore.identity?.getPrincipal().toText() ?? '';

  const copyToClipboard = async () => await navigator.clipboard.writeText(principal);

  const switchRole = () => {
    if (currentRole === 'View') {
      goto('/share');
    } else {
      goto('/feed');
    }
  };
</script>

<div>
  <button class="btn btn-icon variant-ringed" use:popup={popupCombobox}>⋮</button>
  <nav class="w-48 shadow-xl py-2 variant-filled-surface" data-popup="popupCombobox">
    <ul>
      <li>
        <NavBarItem on:click={switchRole}
          >{`Go to ${currentRole === 'View' ? 'publish' : 'view'}`}</NavBarItem
        >
      </li>
      {#if isNullish($authStore.identity)}
        <li>
          <NavBarItem on:click={() => login({ toastStore })}>Login</NavBarItem>
        </li>
      {:else}
        <li>
          <NavBarItem on:click={copyToClipboard}>
            <span><IconCopy /></span>
            <span class="truncate">{principal}</span>
          </NavBarItem>
        </li>
        <li>
          <NavBarItem on:click={() => logout()}>Logout</NavBarItem>
        </li>
      {/if}
    </ul>
  </nav>
</div>
