<script lang="ts">
	export let level: '1' | '2' | '3' | '4' | '5' | '6';

	const component = `h${level}`;
</script>

<svelte:element this={component} class={component}>
	<slot />
</svelte:element>
