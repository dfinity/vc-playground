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
    <!-- Use an "onChange" listener to redirect the user to the selected tab URL. -->
    <select
      bind:value={tabSet}
      id="tabs"
      name="tabs"
      class="block w-full rounded-md border-gray-300 py-2 pl-3 pr-10 text-base focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
    >
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
