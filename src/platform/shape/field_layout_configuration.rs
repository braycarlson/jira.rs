#[derive(Clone, Copy, Debug)]
pub struct FieldLayoutConfiguration;

impl FieldLayoutConfiguration {
    pub const FIELD: crate::Field<Self, crate::Boolean> = crate::Field::new("field");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("required");
}
