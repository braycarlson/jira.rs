#[derive(Clone, Copy, Debug)]
pub struct StatusDetailsScope;

impl StatusDetailsScope {
    pub const PROJECT: crate::Field<Self, crate::Object<super::StatusDetailsScopeProject>> =
        crate::Field::new("project");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
