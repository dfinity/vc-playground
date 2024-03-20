<script lang="ts">
  import { Tab, TabGroup } from '@skeletonlabs/skeleton';

  type Tab = {
    name: string;
    label: string;
    value: number;
  };
  export let tabs: Tab[];
  export let tabSet: number;
</script>

<div class="hidden sm:block">
  <TabGroup justify="justify-center">
    {#each tabs as tab}
      <Tab bind:group={tabSet} name={tab.name} value={tab.value}>{tab.label}</Tab>
    {/each}
    <svelte:fragment slot="panel">
      <slot />
    </svelte:fragment>
  </TabGroup>
</div>
<div class="sm:hidden flex flex-col gap-2">
  <div>
    <label for="tabs" class="sr-only">Select a tab</label>
    <select bind:value={tabSet} id="tabs" name="tabs" class="select">
      {#each tabs as tab}
        <option value={tab.value} id={tab.name}>
          {tab.label}
        </option>
      {/each}
    </select>
  </div>
  <div>
    <slot />
  </div>
</div>
