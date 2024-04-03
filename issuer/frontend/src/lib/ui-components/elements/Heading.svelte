<script lang="ts">
  export let level: '1' | '2' | '3' | '4' | '5' | '6';
  export let align: 'left' | 'center' = 'left';

  const component = `h${level}`;
  let levelClasses = {
    1: 'h1',
    2: 'h2',
    3: 'h3',
    4: 'h4',
    5: 'h5',
    6: 'h6',
  };
  let alignClasses = {
    left: 'text-left',
    center: 'text-center',
  };
</script>

<svelte:element this={component} class={`${levelClasses[level]} ${alignClasses[align]}`}>
  <slot />
</svelte:element>
