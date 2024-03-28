<script lang="ts">
  import { logout } from '$lib/services/auth.services';
  import { authStore } from '$lib/stores/auth.store';
  import { clipboard, popup, type PopupSettings } from '@skeletonlabs/skeleton';
  import IconCopy from './IconCopy.svelte';

  const popupCombobox: PopupSettings = {
    event: 'click',
    target: 'popupCombobox',
    placement: 'bottom',
  };

  let principal: string
  $: principal = $authStore.identity?.getPrincipal().toText() ?? '';
</script>

<div>
  <button class="btn btn-icon bg-initial" use:popup={popupCombobox}>⋮</button>
  <nav class="w-48 shadow-xl py-2 variant-filled-surface" data-popup="popupCombobox">
    <ul>
      <li>
        <button class="btn bg-initial w-full justify-start gap-2" use:clipboard={principal}>
          <span><IconCopy /></span>
          <span class="truncate">{principal}</span>
        </button>
      </li>
      <li>
        <button class="btn bg-initial w-full justify-start" on:click={logout}>Logout</button>
      </li>
    </ul>
  </nav>
</div>
