#[derive(Clone, Copy, Debug)]
pub struct AddGroupBean;

impl AddGroupBean {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
