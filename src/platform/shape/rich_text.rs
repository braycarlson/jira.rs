#[derive(Clone, Copy, Debug)]
pub struct RichText;

impl RichText {
    pub const EMPTY: crate::Field<Self, crate::Boolean> = crate::Field::new("empty");
    pub const EMPTY_ADF: crate::Field<Self, crate::Boolean> = crate::Field::new("emptyAdf");
    pub const FINALISED: crate::Field<Self, crate::Boolean> = crate::Field::new("finalised");
    pub const VALUE_SET: crate::Field<Self, crate::Boolean> = crate::Field::new("valueSet");
}
