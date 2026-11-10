#[derive(Clone, Copy, Debug)]
pub struct Scope;

impl Scope {
    pub const PROJECT: crate::Field<Self, crate::Object<super::ScopeProject>> =
        crate::Field::new("project");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
