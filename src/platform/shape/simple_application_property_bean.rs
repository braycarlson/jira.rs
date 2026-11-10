#[derive(Clone, Copy, Debug)]
pub struct SimpleApplicationPropertyBean;

impl SimpleApplicationPropertyBean {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
