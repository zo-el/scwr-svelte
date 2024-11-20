<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { clientContext } from "../../contexts";
import type { ExecutedInstance } from "./types";

const dispatch = createEventDispatcher();
const client: AppClient = (getContext(clientContext) as any).getClient();

let executionCode: string = "";
let requiredInputs: string = "";
let contractOutputSig: string = "";

$: executionCode, requiredInputs, contractOutputSig;
$: isExecutedInstanceValid = true && executionCode !== "" && requiredInputs !== "" && contractOutputSig !== "";

onMount(() => {
});

async function createExecutedInstance() {
  const executedInstanceEntry: ExecutedInstance = {
    execution_code: executionCode!,
    required_inputs: requiredInputs!,
    contract_output_sig: contractOutputSig!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "scwr",
      zome_name: "runner",
      fn_name: "create_executed_instance",
      payload: executedInstanceEntry,
    });
    dispatch("executed-instance-created", { executedInstanceHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create ExecutedInstance</h3>

  <div>
    <label for="Execution Code">Execution Code</label>
    <textarea name="Execution Code" bind:value={executionCode} required />
  </div>
  <div>
    <label for="Required Inputs">Required Inputs</label>
    <textarea name="Required Inputs" bind:value={requiredInputs} required />
  </div>
  <div>
    <label for="Contract Output Sig">Contract Output Sig</label>
    <textarea
      name="Contract Output Sig"
      bind:value={contractOutputSig}
      required
    />
  </div>

  <button
    disabled={!isExecutedInstanceValid}
    on:click={() => createExecutedInstance()}
  >
    Create ExecutedInstance
  </button>
</div>
