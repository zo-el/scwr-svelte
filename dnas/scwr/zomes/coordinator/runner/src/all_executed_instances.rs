use hdk::prelude::*;
use runner_integrity::*;

#[hdk_extern]
pub fn get_all_executed_instances() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_executed_instances");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllExecutedInstances)?
            .build(),
    )
}
