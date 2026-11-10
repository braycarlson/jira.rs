#[derive(Clone, Copy, Debug)]
pub struct TargetMandatoryFields;

impl TargetMandatoryFields {
    pub const FIELDS: crate::Field<Self, crate::Map<crate::Object<super::MandatoryFields>>> =
        crate::Field::new("fields");
}
