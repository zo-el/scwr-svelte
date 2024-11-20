use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ExecutedInstance {
    pub execution_code: String,
    pub required_inputs: String,
    pub contract_output_sig: String,
}

pub fn validate_create_executed_instance(
    _action: EntryCreationAction,
    _executed_instance: ExecutedInstance,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_executed_instance(
    _action: Update,
    _executed_instance: ExecutedInstance,
    _original_action: EntryCreationAction,
    _original_executed_instance: ExecutedInstance,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Executed Instances cannot be updated".to_string(),
    ))
}

pub fn validate_delete_executed_instance(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_executed_instance: ExecutedInstance,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Executed Instances cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_executed_instances(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _executed_instance: crate::ExecutedInstance = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_executed_instances(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
