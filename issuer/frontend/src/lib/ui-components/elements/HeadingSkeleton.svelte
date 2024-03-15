<script lang="ts">
  type Size = 'sm' | 'md' | 'lg';
  export let size: Size = 'md';

  let heightClassNames: Record<Size, string> = {
    sm: 'min-h-6',
    md: 'min-h-8',
    lg: 'min-h-12',
  };
</script>

<div class={`placeholder ${heightClassNames[size]} max-w-36`} />
