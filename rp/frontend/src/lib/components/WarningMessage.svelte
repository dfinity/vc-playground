<script lang="ts">
  import WarningIcon from './WarningIcon.svelte';
</script>

<p class="flex items-center gap-1 text-warning-700-200-token text-center">
  <WarningIcon />
  <span><slot /></span>
</p>
