<script lang="ts">
	type Variant = 'success' | 'error' | 'tertiary' | 'ghost' | 'primary';
	export let variant: Variant;
	export let testId: string | undefined = undefined;

	let variantClasses: Record<Variant, string> = {
		success: 'variant-filled-success',
		error: 'variant-filled-error',
		tertiary: 'variant-filled-tertiary',
		ghost: 'variant-ghost',
		primary: 'variant-filled',
	};
</script>

<button on:click type="button" class={`btn ${variantClasses[variant]}`} data-tid={testId}
	><slot /></button
>
