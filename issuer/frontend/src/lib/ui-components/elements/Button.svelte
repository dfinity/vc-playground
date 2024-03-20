<script lang="ts">
  import Spinner from './Spinner.svelte';

  type Variant = 'success' | 'error' | 'ghost' | 'primary' | 'secondary';
  export let variant: Variant;
  export let testId: string | undefined = undefined;
  type Size = 'sm' | 'md' | 'lg';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let href: string | undefined = undefined;
  export let loading: boolean = false;
  export let disabled: boolean = false;

  let sizeClass: Record<Size, string> = {
    sm: 'btn-sm',
    md: 'btn-md',
    lg: 'btn-lg',
  };
  let variantClasses: Record<Variant, string> = {
    success: 'variant-filled-success',
    error: 'variant-filled-error',
    ghost: 'variant-ghost',
    primary: 'variant-filled',
    secondary: 'variant-filled-secondary',
  };
</script>

{#if href}
  <a {href} target="_blank" class={`btn ${sizeClass[size]} ${variantClasses[variant]}`}><slot /></a>
{:else}
  <button
    disabled={loading || disabled}
    on:click
    type="button"
    class={`flex gap-2 btn ${sizeClass[size]} ${variantClasses[variant]}`}
    data-tid={testId}
  >
    {#if loading}
      <Spinner />
    {/if}
    <slot />
  </button>
{/if}
