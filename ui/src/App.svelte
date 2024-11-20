<script lang="ts">
import type { ActionHash, AppClient, HolochainError } from "@holochain/client";
import { AppWebsocket } from "@holochain/client";
import { onMount, setContext } from "svelte";
import CreateExecutedInstance from "./scwr/runner/CreateExecutedInstance.svelte"
// import ExecutedInstanceDetails from "./scwr/runner/ExecutedInstanceDetails.svelte"
import AllExecutedInstances from "./scwr/runner/AllExecutedInstances.svelte"
import logo from "./assets/holochainLogo.svg";
import { clientContext } from "./contexts";

let client: AppClient | undefined;
let error: HolochainError | undefined;
let loading = false;

onMount(async () => {
  try {
    loading = true;
    client = await AppWebsocket.connect();
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
});

setContext(clientContext, {
  getClient: () => client,
});
</script>

<main>
  <div>
    <a href="https://developer.holochain.org/get-started/" target="_blank">
      <img src={logo} class="logo holochain" alt="holochain logo" />
    </a>
  </div>
  <h1>Holochain Svelte hApp</h1>
  <div>
    <div class="card">
      {#if loading}
        <p>connecting...</p>
      {:else if error}
        <p>{error.message}</p>
      {:else}
        <p>Client is connected.</p>
      {/if}
    </div>
    <p>
      Import scaffolded components into <code>src/App.svelte</code> to use your
      hApp
    </p>
    <CreateExecutedInstance/>
    <AllExecutedInstances />
    <p class="read-the-docs">Click on the Holochain logo to learn more</p>
  </div>
</main>

<style>
.logo {
  height: 15em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
  width: auto;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.holochain:hover {
  filter: drop-shadow(0 0 2em #61dafbaa);
}

.card {
  padding: 2em;
}

.read-the-docs {
  color: #888;
}
</style>
