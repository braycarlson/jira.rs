#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldAssociationSchemeResponse;

impl UpdateFieldAssociationSchemeResponse {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const LINKS: crate::Field<
        Self,
        crate::Object<super::UpdateFieldAssociationSchemeLinksBean>,
    > = crate::Field::new("links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
