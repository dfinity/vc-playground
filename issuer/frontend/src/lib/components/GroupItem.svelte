<script lang="ts">
	import AvatarSize from '$lib/ui-components/elements/AvatarSize.svelte';
	import Badge from '$lib/ui-components/elements/Badge.svelte';
	import Button from '$lib/ui-components/elements/Button.svelte';
	import ListItem from '$lib/ui-components/elements/ListItem.svelte';
	import type { MembershipStatus, PublicGroupData } from '../../declarations/meta_issuer.did';

	export let group: PublicGroupData;

	let isNotMember: boolean;
	$: isNotMember = group.membership_status.length === 0 || 'Rejected' in group.membership_status[0];

	const statusVariant = (status: MembershipStatus | undefined): 'success' | 'default' => {
		if (status === undefined || 'Rejected' in status) {
			throw new Error('It should not show a badge');
		}
		if ('Accepted' in status) return 'success';
		// Only missing 'PendingReview'
		return 'default';
	};
	const badgeText = (status: MembershipStatus | undefined): string => {
		if (status === undefined || 'Rejected' in status) {
			throw new Error('It should not show a badge');
		}
		if ('Accepted' in status) return '🪪 Member';
		// Only missing 'PendingReview'
		return '📤 Pending';
	};
</script>

<ListItem>
	<AvatarSize num={group.stats.member_count} slot="start" />
	<svelte:fragment slot="main">{group.group_name}</svelte:fragment>
	<svelte:fragment slot="end">
		{#if isNotMember}
			<Button variant="primary" size="sm">Join</Button>
		{:else if group.is_owner[0]}
			<Badge variant="primary">👑 Owner</Badge>
		{:else}
			<Badge variant={statusVariant(group.membership_status[0])}
				>{badgeText(group.membership_status[0])}</Badge
			>
		{/if}
	</svelte:fragment>
</ListItem>
