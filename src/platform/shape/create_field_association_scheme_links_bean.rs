#[derive(Clone, Copy, Debug)]
pub struct CreateFieldAssociationSchemeLinksBean;

impl CreateFieldAssociationSchemeLinksBean {
    pub const ASSOCIATIONS: crate::Field<Self, crate::Text> = crate::Field::new("associations");
    pub const PROJECTS: crate::Field<Self, crate::Text> = crate::Field::new("projects");
}
