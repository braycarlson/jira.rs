#[derive(Clone, Copy, Debug)]
pub struct FieldSchemePayload;

impl FieldSchemePayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ITEMS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldAssociationItemPayload>>,
    > = crate::Field::new("items");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
