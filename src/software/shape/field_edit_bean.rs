#[derive(Clone, Copy, Debug)]
pub struct FieldEditBean;

impl FieldEditBean {
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
