#[derive(Clone, Copy, Debug)]
pub struct DefaultShareScope;

impl DefaultShareScope {
    pub const SCOPE: crate::Field<Self, crate::Text> = crate::Field::new("scope");
}
