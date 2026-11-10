#[derive(Clone, Copy, Debug)]
pub struct UpdateDefaultProjectClassificationBean;

impl UpdateDefaultProjectClassificationBean {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
