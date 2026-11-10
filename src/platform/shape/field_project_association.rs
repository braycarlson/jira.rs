#[derive(Clone, Copy, Debug)]
pub struct FieldProjectAssociation;

impl FieldProjectAssociation {
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
