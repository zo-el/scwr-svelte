<script lang="ts">
import type {
  ActionHash,
  AgentPubKey,
  AppClient,
  EntryHash,
  HolochainError,
  Link,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { SignalType } from "@holochain/client";
import { getContext, onMount } from "svelte";
import { clientContext } from "../../contexts";
import ExecutedInstanceDetail from "./ExecutedInstanceDetail.svelte";
import type { RunnerSignal } from "./types";

const client: AppClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> = [];
let loading = false;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  await fetchExecutedInstances();
  client.on("signal", signal => {
    if (signal.zome_name !== "runner") return;
    const payload = signal.payload as RunnerSignal;
    if (payload.type !== "EntryCreated") return;
    if (payload.app_entry.type !== "ExecutedInstance") return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchExecutedInstances() {
  loading = true;
  try {
    console.log(">>>>>>>>>>", client);
    
    const links: Array<Link> = await client.callZome({
      cap_secret: null,
      role_name: "scwr",
      zome_name: "runner",
      fn_name: "get_all_executed_instances",
      payload: null,
    });
    if (links.length) {
      hashes = links.map(l => l.target);
    }
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">
    Error fetching the executed instances: {error.message}.
  </div>
{:else if !hashes.length}
  <div class="alert">No executed instances found.</div>
{:else}
  <div>
    {#each hashes as hash}
      <ExecutedInstanceDetail
        executedInstanceHash={hash}
        on:executed-instance-deleted={() => fetchExecutedInstances()}
      />
    {/each}
  </div>
{/if}
