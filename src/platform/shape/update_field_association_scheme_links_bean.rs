#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldAssociationSchemeLinksBean;

impl UpdateFieldAssociationSchemeLinksBean {
    pub const ASSOCIATIONS: crate::Field<Self, crate::Text> = crate::Field::new("associations");
    pub const PROJECTS: crate::Field<Self, crate::Text> = crate::Field::new("projects");
}
