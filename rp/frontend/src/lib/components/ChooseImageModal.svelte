<script lang="ts">
	import { getModalStore } from '@skeletonlabs/skeleton';

	/* eslint-disable-next-line */
	export let parent: any;
	const modalStore = getModalStore();

	const images = [
		'https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1597077962467-be16edcc6a43?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2MzZ8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1553184570-557b84a3a308?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NTF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1509130446053-899ae7358ce6?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NjF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1620005839871-7ac4aed5ddbc?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NzN8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1597531072931-8fceba101e4e?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2OTB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1510111652602-195fc654aa83?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY0Nzl8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1612145342709-eadb6e22acca?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY3MDh8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
		'https://images.unsplash.com/photo-1597077917598-97ca3922a317?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY3MjF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop'
	];

	const selectImageFactory = (image: string) => (evt: Event) => {
		evt.preventDefault();
		$modalStore[0].response?.(image);
		parent.onClose();
	};
</script>

<div
	class="modal block overflow-y-auto bg-surface-100-800-token w-modal h-auto p-4 space-y-4 rounded-container-token shadow-xl"
	role="dialog"
	aria-modal="true"
	aria-label="Choose Image"
>
	<header class="modal-header text-2xl font-bold">Choose Image</header>
	<article class="grid grid-cols-2 md:grid-cols-3 gap-4">
		{#each images as image}
			<a href={'#'} on:click={selectImageFactory(image)}>
				<img class="h-auto max-w-full rounded-lg" src={image} alt="Option 1" />
			</a>
		{/each}
	</article>
</div>
