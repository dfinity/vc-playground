<script lang="ts">
  export let align: 'left' | 'center' | 'right' = 'left';

  const alignClasses = {
    left: 'text-left',
    center: 'text-center',
    right: 'text-right',
  };
</script>

<p class={alignClasses[align]}>
  <slot />
</p>
