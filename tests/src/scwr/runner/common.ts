import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeEntryHash,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";

export async function sampleExecutedInstance(cell: CallableCell, partialExecutedInstance = {}) {
  return {
    ...{
      execution_code: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      required_inputs: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      contract_output_sig: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    },
    ...partialExecutedInstance,
  };
}

export async function createExecutedInstance(cell: CallableCell, executedInstance = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "runner",
    fn_name: "create_executed_instance",
    payload: executedInstance || await sampleExecutedInstance(cell),
  });
}
