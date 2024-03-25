<script lang="ts">
  import ImagesGrid from '$lib/components/ImagesGrid.svelte';

  const contentImages: { imageUrl: string; issuerName: string }[] = [
    'https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1597077962467-be16edcc6a43?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2MzZ8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1553184570-557b84a3a308?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NTF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1509130446053-899ae7358ce6?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NjF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1620005839871-7ac4aed5ddbc?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NzN8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1597531072931-8fceba101e4e?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2OTB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1510111652602-195fc654aa83?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY0Nzl8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1612145342709-eadb6e22acca?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY3MDh8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
    'https://images.unsplash.com/photo-1597077917598-97ca3922a317?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY3MjF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  ].map((imageUrl, i) => ({ issuerName: `Photo Gallery with a very long name ${i}`, imageUrl }));
</script>

<h1 class="h1 text-center">View and Share Content</h1>
<p class="text-center">
  This is an example of a Relying Party. You can view an image if you hold the particular credential
  required to access the image.
</p>
<ImagesGrid images={contentImages} />
