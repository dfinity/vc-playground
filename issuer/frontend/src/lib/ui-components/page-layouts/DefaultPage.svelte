<script>
  import Heading from '../elements/Heading.svelte';
</script>

<section class="flex flex-col gap-8">
  <slot name="nav" />
  <slot name="callout" />
  <Heading align="middle" level="1">
    <slot name="title" />
  </Heading>
  <div class="flex flex-col gap-6">
    <slot />
  </div>
</section>
