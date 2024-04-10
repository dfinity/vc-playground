<script lang="ts">
  import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

  const popupCombobox: PopupSettings = {
    event: 'click',
    target: 'popupCombobox',
    placement: 'bottom',
  };
</script>

<div>
  <button class="btn btn-icon variant-ringed" use:popup={popupCombobox}>⋮</button>
  <nav class="w-48 shadow-xl py-2 variant-filled-surface" data-popup="popupCombobox">
    <slot />
  </nav>
</div>
