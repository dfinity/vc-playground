<script lang="ts">
  import { logout } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import IconCopy from './IconCopy.svelte';
  import PopoverDropdown from '$lib/ui-components/elements/PopoverDropdown.svelte';
  import NavBarItem from '$lib/ui-components/elements/NavBarItem.svelte';

  let principal: string;
  $: principal = $authStore.identity?.getPrincipal().toText() ?? '';

  const copyToClipboard = async () => await navigator.clipboard.writeText(principal);
</script>

<PopoverDropdown>
  <ul>
    <li>
      <NavBarItem on:click={copyToClipboard}>
        <span><IconCopy /></span>
        <span class="truncate">{principal}</span>
      </NavBarItem>
    </li>
    <li>
      <NavBarItem on:click={logout}>Logout</NavBarItem>
    </li>
  </ul>
</PopoverDropdown>
