#[derive(Clone, Copy, Debug)]
pub struct ColorBean;

impl ColorBean {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
