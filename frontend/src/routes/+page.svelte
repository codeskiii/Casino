<!-- src/routes/example/+page.svelte -->

<script lang="ts">
  export let data;

  let games = data["props"]["data"]["games"]; 
  let lotteries = data["props"]["data"]["lotteries"];

</script>

<div>
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

    <div class="uk-grid uk-child-width-1-3@m uk-grid-match uk-flex-center uk-card-small">
      {#each games as game}
          <div>
              <div class="uk-card uk-card-default uk-card-hover uk-card-body uk-margin-medium-top">
                  <h3 class="uk-card-title">{game.name}</h3>
                  <p>{game.description}</p>
                  <p class="uk-text-bold">Entry cost: {game.entry_cost}</p>
                  <p class="uk-text-bold">Max multiplier: {game.prize_multiplier}x</p>
              </div>
          </div>
      {/each}
  </div>

  {:else}
  <p>Failed Loading</p>
  {/if}
</div>

<!-- 
<pre>{JSON.stringify(data, null, 2)}</pre>
-->