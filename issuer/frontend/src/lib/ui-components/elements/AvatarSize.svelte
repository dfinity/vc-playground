<script lang="ts">
	export let num: number;

	let displayNum: string;
	$: displayNum = num > 1_000 ? '999+' : num.toString();
</script>

<figure
	class="avatar flex aspect-square text-surface-50 font-semibold justify-center items-center overflow-hidden isolate bg-primary-500 w-12 rounded-full"
>
	<svg class="avatar-initials w-full h-full" viewBox="0 0 512 512"
		><text
			x="50%"
			y="50%"
			dominant-baseline="central"
			text-anchor="middle"
			font-weight="bold"
			font-size="150"
			class="avatar-text fill-token">{displayNum}</text
		></svg
	>
</figure>
