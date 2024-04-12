<script lang="ts">
  export let testId: string | undefined = undefined;
  export let href: string;
</script>

<a
  {href}
  data-tid={testId}
  class="card variant-filled card-hover p-4 flex flex-col justify-center items-center"
>
  <div class="sm:p-8 p-4 flex justify-center items-center">
    <slot name="icon" />
  </div>
  <h3 class="h3 text-center"><slot name="title" /></h3>
  <p class="text-center"><slot /></p>
</a>
