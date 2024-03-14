<script lang="ts">
	import Heading from './Heading.svelte';
</script>

<li class="flex justify-between items-center">
	<Heading level="5"><slot name="main" /></Heading>
	<span class="flex gap-2 align-center">
		<slot name="end" />
	</span>
</li>
