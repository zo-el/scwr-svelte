<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { clientContext } from "../../contexts";
import type { ExecutedInstance } from "./types";

const client: AppClient = (getContext(clientContext) as any).getClient();
const dispatch = createEventDispatcher();

let loading: boolean = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let executedInstance: ExecutedInstance | undefined;

export let executedInstanceHash: ActionHash;

$: error, loading, record, executedInstance;

onMount(async () => {
  if (executedInstanceHash === undefined) {
    throw new Error(`The executedInstanceHash input is required for the ExecutedInstanceDetail element`);
  }
  await fetchExecutedInstance();
});

async function fetchExecutedInstance() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "scwr",
      zome_name: "runner",
      fn_name: "get_executed_instance",
      payload: executedInstanceHash,
    });
    if (record) {
      executedInstance = decode((record.entry as any).Present.entry) as ExecutedInstance;
    }
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
  <div class="alert">Error fetching the executed instance: {error.message}</div>
{:else}
  <section>
    <div>
      <span><strong>Execution Code:</strong></span>
      <span>{executedInstance?.execution_code}</span>
    </div>
    <div>
      <span><strong>Required Inputs:</strong></span>
      <span>{executedInstance?.required_inputs}</span>
    </div>
    <div>
      <span><strong>Contract Output Sig:</strong></span>
      <span>{executedInstance?.contract_output_sig}</span>
    </div>

    <div></div>
  </section>
{/if}
