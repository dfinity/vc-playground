<script lang="ts">
  export let testId: string | undefined = undefined;
</script>

<ul data-tid={testId} role="table" class="flex flex-col">
  <slot />
</ul>
