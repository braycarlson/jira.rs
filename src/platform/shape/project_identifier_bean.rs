#[derive(Clone, Copy, Debug)]
pub struct ProjectIdentifierBean;

impl ProjectIdentifierBean {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
