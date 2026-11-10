#[derive(Clone, Copy, Debug)]
pub struct AddFieldBean;

impl AddFieldBean {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
