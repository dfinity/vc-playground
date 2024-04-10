<script lang="ts">
  import WarningIcon from '../icons/WarningIcon.svelte';
</script>

<p class="flex items-center gap-1 text-warning-500-400-token text-center">
  <WarningIcon />
  <span><slot /></span>
</p>
