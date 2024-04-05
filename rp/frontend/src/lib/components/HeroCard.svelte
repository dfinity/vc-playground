<script lang="ts">
  export let testId: string | undefined = undefined;
</script>

<button
  data-tid={testId}
  class="card card-hover p-4 flex flex-col justify-center items-center"
  on:click
>
  <div class="p-8 flex justify-center items-center">
    <slot name="icon" />
  </div>
  <h3 class="h3 text-center"><slot name="title" /></h3>
  <p class="text-center"><slot /></p>
</button>
