<script lang="ts">
	type Variant = 'success' | 'default';
	export let variant: Variant;

	let variantClasses: Record<Variant, string> = {
		success: 'variant-filled-success',
		default: 'variant-filled-surface',
	};
</script>

<span class={`badge ${variantClasses[variant]}`}><slot /></span>
