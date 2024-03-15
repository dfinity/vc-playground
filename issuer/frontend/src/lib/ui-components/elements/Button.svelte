<script lang="ts">
  type Variant = 'success' | 'error' | 'tertiary' | 'ghost' | 'primary' | 'secondary';
  export let variant: Variant;
  export let testId: string | undefined = undefined;
  export let size: 'sm' | 'md' | 'lg' = 'md';

  let variantClasses: Record<Variant, string> = {
    success: 'variant-filled-success',
    error: 'variant-filled-error',
    tertiary: 'variant-filled-tertiary',
    ghost: 'variant-ghost',
    primary: 'variant-filled',
    secondary: 'variant-filled-secondary',
  };
</script>

<button
  on:click
  type="button"
  class={`btn btn-${size} ${variantClasses[variant]}`}
  data-tid={testId}><slot /></button
>
