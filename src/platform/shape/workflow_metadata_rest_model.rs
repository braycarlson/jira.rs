#[derive(Clone, Copy, Debug)]
pub struct WorkflowMetadataRestModel;

impl WorkflowMetadataRestModel {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const VERSION: crate::Field<Self, crate::Object<super::DocumentVersion>> =
        crate::Field::new("version");
}
