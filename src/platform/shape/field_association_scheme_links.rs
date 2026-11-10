#[derive(Clone, Copy, Debug)]
pub struct FieldAssociationSchemeLinks;

impl FieldAssociationSchemeLinks {
    pub const ASSOCIATIONS: crate::Field<Self, crate::Text> = crate::Field::new("associations");
    pub const PROJECTS: crate::Field<Self, crate::Text> = crate::Field::new("projects");
}
