<script lang="ts">
  import { goto } from '$app/navigation';
  import { ISSUER_PARAM } from '$lib/constants/url-params.constants';
  import type { PublicGroupData } from '../../declarations/meta_issuer.did';
  import IssuerItem from '$lib/components/IssuerItem.svelte';

  export let issuer: PublicGroupData;

  const onClick = () => {
    goto(`/issuers/?${ISSUER_PARAM}=${encodeURIComponent(issuer.group_name)}`);
  };
</script>

<IssuerItem {onClick} {issuer} />
