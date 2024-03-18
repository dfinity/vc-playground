<script lang="ts">
  import AvatarSkeleton from '$lib/ui-components/elements/AvatarSkeleton.svelte';
  import ButtonSkeleton from '$lib/ui-components/elements/ButtonSkeleton.svelte';
  import ListItem from '$lib/ui-components/elements/ListItem.svelte';
</script>

<ListItem>
  <AvatarSkeleton slot="start" />
  <div class="placeholder" slot="main" />
  <ButtonSkeleton slot="end" />
</ListItem>
