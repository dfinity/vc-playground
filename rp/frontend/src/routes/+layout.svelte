<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import MainWrapper from '$lib/components/MainWrapper.svelte';
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <a href="/" class="text-xl uppercase font-bold" aria-label="Go to Home" slot="lead"
        >Relying Party
      </a>
      <svelte:fragment slot="trail">
        <Button variant="primary">Share Image</Button>
        <Button variant="secondary">Login</Button>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
	<MainWrapper>
		<!-- Page Route Content -->
		<slot />
	</MainWrapper>
</AppShell>
