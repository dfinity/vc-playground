<script lang="ts">
  export let gap: 'sm' | 'md' | 'l' = 'sm';
  export let align: 'start' | 'center' | 'end' | 'stretch' = 'stretch';

  let gapClasses = {
    sm: 'gap-2',
    md: 'gap-4',
    l: 'sm:gap-12 gap-6',
  };
  let alignClasses = {
    start: 'items-start',
    center: 'items-center',
    end: 'items-end',
    stretch: 'items-stretch',
  };
</script>

<div class={`flex flex-col ${gapClasses[gap]} ${alignClasses[align]}`}>
  <slot />
</div>
