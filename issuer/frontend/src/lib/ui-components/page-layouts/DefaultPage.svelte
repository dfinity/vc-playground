<script>
  import Heading from '../elements/Heading.svelte';
</script>

<section class="flex flex-col gap-8">
  <slot name="nav" />
  <slot name="callout" />
  <Heading align="center" testId="page-title" level="1">
    <slot name="title" />
  </Heading>
  <Heading align="center" level="3">
    <slot name="subtitle" />
  </Heading>
  <div class="flex flex-col gap-6">
    <slot />
  </div>
</section>
