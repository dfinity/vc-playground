<script lang="ts">
	import MainWrapper from '$lib/components/MainWrapper.svelte';
import '../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';
</script>

<!-- App Shell -->
<AppShell>
	<MainWrapper>
		<!-- Page Route Content -->
		<slot />
	</MainWrapper>
</AppShell>
