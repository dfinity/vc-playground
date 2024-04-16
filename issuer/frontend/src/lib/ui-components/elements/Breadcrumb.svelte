<script lang="ts">
  export let paths: { name: string; href?: string }[];
</script>

<ol class="breadcrumb">
  {#each paths as path, index}
    <li class="crumb">
      {#if path.href}
        <a class="anchor" href={path.href}>{path.name}</a>
      {:else}
        {path.name}
      {/if}
    </li>
    {#if index < paths.length - 1}
      <li class="crumb-separator" aria-hidden>&rsaquo;</li>
    {/if}
  {/each}
</ol>
