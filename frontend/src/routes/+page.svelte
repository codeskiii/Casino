<!-- src/routes/example/+page.svelte -->
<script>
    export let data;
  </script>
  
  <h1>API Data</h1>
  <pre>{JSON.stringify(data, null, 2)}</pre>