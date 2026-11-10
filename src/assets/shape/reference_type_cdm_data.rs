#[derive(Clone, Copy, Debug)]
pub struct ReferenceTypeCdmData;

impl ReferenceTypeCdmData {
    pub const TYPES: crate::Field<Self, crate::List<crate::Object<super::ReferenceTypeCdmType>>> =
        crate::Field::new("types");
}
