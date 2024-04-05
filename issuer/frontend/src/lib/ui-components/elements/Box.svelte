<script lang="ts">
  export let responsive = true;
</script>

<span
  class={`flex ${responsive ? 'sm:gap-8' : 'gap-2'} sm:flex-row ${responsive ? 'flex-col' : 'flex-row'} items-center`}
  ><slot /></span
>
