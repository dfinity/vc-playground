<script lang="ts">
	import { goto } from '$app/navigation';
	import { login } from '$lib/services/login.services';
	import Button from '$lib/ui-components/elements/Button.svelte';
	import '../../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';

	const loginUser = async () => {
		await login();
		goto('/home');
	};
</script>

<AppShell>
	<div class="relative isolate px-6 pt-14 lg:px-8">
		<div
			class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56 flex justify-center items-center flex-col gap-8"
		>
			<div class="text-center">
				<h1 class="text-4xl font-bold tracking-tight sm:text-6xl">Welcome to VC Playground</h1>
			</div>
			<Button variant="primary" testId="login-button" on:click={loginUser}
				>Connect With Internet Identity</Button
			>
		</div>
	</div>
</AppShell>
