<!-- src/routes/example/+page.svelte -->
<script lang="ts">
  export let data;

  let games = data["props"]["data"]["games"]; 
  let lotteries = data["props"]["data"]["lotteries"];

  console.log(games);
</script>
 
{#if games.length > 0}
  <!-- <ul> 
    {#each games as game, index}
      <li>
        <h2>{index + 1}. {game.name}</h2>
        <p>{game.description}</p>
        <ul>
          {#each game.cryptos as crypto}
            <li>{crypto}</li>
          {/each}
        </ul>
      </li>
    {/each}
  </ul> -->

  <div class="uk-grid">
    {#each games as game}

    <div class="uk-card uk-card-default uk-card-hover uk-card-body uk-margin-medium-top">
      <h3 class="uk-card-title">{game.name}</h3>
      <p>{game.description}</p>
      <p class="uk-text-bold">Entry cost: {game.entry_cost}</p>
      <p class="uk-text-bold">Max multiplier: {game.prize_multiplier}x</p>
    </div>

    {/each}
  </div>

{:else}
  <p>Failed Loading</p>
{/if}

<!-- 
<pre>{JSON.stringify(data, null, 2)}</pre>
-->