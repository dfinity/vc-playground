<script lang="ts">
  import ButtonSkeleton from '$lib/ui-components/elements/ButtonSkeleton.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
</script>

<ListItem>
  <div class="placeholder" slot="main" />
  <ButtonSkeleton slot="end" />
</ListItem>
