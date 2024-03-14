<script lang="ts">
	import Heading from './Heading.svelte';
</script>

<li class="flex justify-between items-center gap-4">
	<slot name="start" />
	<div class="flex-1">
		<Heading level="5"><slot name="main" /></Heading>
	</div>
	<span class="flex gap-2 align-center">
		<slot name="end" />
	</span>
</li>


