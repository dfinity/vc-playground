<script lang="ts">
	type Variant = 'success' | 'default' | 'primary';
	export let variant: Variant;

	let variantClasses: Record<Variant, string> = {
		success: 'variant-filled-success',
		primary: 'variant-filled-primary',
		default: 'variant-filled-surface',
	};
</script>

<span class={`badge ${variantClasses[variant]}`}><slot /></span>
