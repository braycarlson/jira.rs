#[derive(Clone, Copy, Debug)]
pub struct FieldLayoutPayload;

impl FieldLayoutPayload {
    pub const CONFIGURATION: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldLayoutConfiguration>>,
    > = crate::Field::new("configuration");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
