<script>
  import Heading from '../elements/Heading.svelte';
</script>

<section class="flex flex-col gap-8">
  <Heading level="1">
    <slot name="title" />
  </Heading>
  <div class="flex flex-col gap-6">
    <slot />
  </div>
</section>
