<script lang="ts">
  import ImagesGrid from '$lib/components/ImagesGrid.svelte';

  let contentImages: string[] = Array.from({ length: 10 }, (_, i) => `Test ${i}`);
</script>

<h1 class="h1 text-center">View and Share Content</h1>
<p class="text-center">
  This is an example of a Relying Party. You can view an image if you hold the particular credential
  required to access the image.
</p>
<ImagesGrid images={contentImages} />
