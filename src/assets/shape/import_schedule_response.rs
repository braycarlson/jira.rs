#[derive(Clone, Copy, Debug)]
pub struct ImportScheduleResponse;

impl ImportScheduleResponse {
    pub const COLLECTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("collectionId");
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IMPORT_SOURCE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("importSourceId");

    pub const RUN_INTERVAL: crate::Field<Self, crate::Text> = crate::Field::new("runInterval");
    pub const START_TIME: crate::Field<Self, crate::Text> = crate::Field::new("startTime");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
