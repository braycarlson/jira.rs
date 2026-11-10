#[derive(Clone, Copy, Debug)]
pub struct GetFieldAssociationSchemeByIDResponse;

impl GetFieldAssociationSchemeByIDResponse {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const FIELDS_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("fieldsCount");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");

    pub const LINKS: crate::Field<Self, crate::Object<super::FieldAssociationSchemeLinks>> =
        crate::Field::new("links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
