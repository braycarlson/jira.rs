#[derive(Clone, Copy, Debug)]
pub struct WorkflowStatusPayload;

impl WorkflowStatusPayload {
    pub const LAYOUT: crate::Field<Self, crate::Object<super::WorkflowStatusLayoutPayload>> =
        crate::Field::new("layout");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("properties");
}
