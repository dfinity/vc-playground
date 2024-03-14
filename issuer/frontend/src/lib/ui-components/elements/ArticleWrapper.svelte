<script>
	import Heading from './Heading.svelte';
</script>

<article class="flex flex-col gap-4">
	<Heading level="3">
		<slot name="title" />
	</Heading>
	<div>
		<slot />
	</div>
</article>
