<svelte:head>
  <title>Title</title>
</svelte:head>

<p>Main page</p>
{#each Array(25) as _, i}
  <p>{i + 1}</p>
{/each}
