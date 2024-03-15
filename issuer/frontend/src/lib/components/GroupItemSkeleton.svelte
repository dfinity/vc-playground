<script lang="ts">
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
</script>

<ListItem>
  <div class="placeholder-circle w-12" slot="start" />
  <div class="placeholder" slot="main" />
  <div class="w-16" slot="end">
    <div class="placeholder" />
  </div>
</ListItem>
