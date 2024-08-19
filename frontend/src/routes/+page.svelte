<!-- src/routes/example/+page.svelte -->
<script lang="ts">
  export let data;
  let games = data["props"]["data"]["games"];
</script>

<h1>API Data</h1>
 
{#if games.length > 0}
  <ul>
    {#each games as game, index}
      <li>
        <h2>{index + 1}. {game.name}</h2> <!-- Wyświetlanie indeksu i nazwy gry -->
        <p>{game.description}</p>
        <ul>
          {#each game.cryptos as crypto}
            <li>{crypto}</li>
          {/each}
        </ul>
      </li>
    {/each}
  </ul>
{:else}
  <p>No data available.</p>
{/if}

<!-- 
<pre>{JSON.stringify(data, null, 2)}</pre>
-->