#[derive(Clone, Copy, Debug)]
pub struct StatusMappingDTO;

impl StatusMappingDTO {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");

    pub const STATUS_MIGRATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::StatusMigration>>,
    > = crate::Field::new("statusMigrations");
}
