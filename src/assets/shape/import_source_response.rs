#[derive(Clone, Copy, Debug)]
pub struct ImportSourceResponse;

impl ImportSourceResponse {
    pub const COLLECTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("collectionId");
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const DATE_FORMAT: crate::Field<Self, crate::Text> = crate::Field::new("dateFormat");

    pub const DATE_TIME_FORMAT: crate::Field<Self, crate::Text> =
        crate::Field::new("dateTimeFormat");

    pub const DEFAULT_CONCATENATOR: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultConcatenator");

    pub const DEFAULT_HANDLE_COMPUTE_ISSUE_VALUES: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultHandleComputeIssueValues");

    pub const DEFAULT_HANDLE_EMPTY_VALUES: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultHandleEmptyValues");

    pub const DEFAULT_HANDLE_NOT_MAPPED_VALUES: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultHandleNotMappedValues");

    pub const DEFAULT_HANDLE_NULL_VALUES: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultHandleNullValues");

    pub const DEFAULT_HANDLE_UNKNOWN_VALUES: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultHandleUnknownValues");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IMPORT_EXECUTION_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("importExecutionType");

    pub const IMPORT_SOURCE_MODULE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("importSourceModuleKey");

    pub const IMPORT_SOURCE_OT_ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::ImportSourceResponseImportSourceOTEntries>>,
    > = crate::Field::new("importSourceOTEntries");

    pub const IMPORT_SPECIFIC_CONFIGURATION: crate::Field<Self, crate::Text> =
        crate::Field::new("importSpecificConfiguration");

    pub const IMPORT_STATUS: crate::Field<
        Self,
        crate::Object<super::ImportSourceResponseImportStatus>,
    > = crate::Field::new("importStatus");

    pub const INTEGRATED_IMPORT_TYPE_EXTENSION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("integratedImportTypeExtensionId");

    pub const INTEGRATED_IMPORT_TYPE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("integratedImportTypeId");

    pub const IS_IMPORT_SOURCE_SCHEDULING_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isImportSourceSchedulingEnabled");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");

    pub const SCHEDULED_IMPORT_DETAILS: crate::Field<
        Self,
        crate::Object<super::ScheduledImportDetails>,
    > = crate::Field::new("scheduledImportDetails");

    pub const TOKEN_GENERATED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("tokenGenerated");

    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
