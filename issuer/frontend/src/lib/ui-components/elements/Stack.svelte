<script lang="ts">
  export let gap: 'sm' | 'l' = 'sm';

  let gapClasses = {
    sm: 'gap-2',
    l: 'sm:gap-12 gap-6',
  };
</script>

<div class={`flex flex-col ${gapClasses[gap]}`}>
  <slot />
</div>
