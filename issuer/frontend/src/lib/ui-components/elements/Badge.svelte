<script lang="ts">
  type Variant = 'success' | 'default' | 'primary' | 'error';
  export let variant: Variant;

  let variantClasses: Record<Variant, string> = {
    success: 'variant-filled-success',
    primary: 'variant-filled-primary',
    default: 'variant-filled-surface',
    error: 'variant-filled-error',
  };
</script>

<span class={`badge ${variantClasses[variant]}`}><slot /></span>
