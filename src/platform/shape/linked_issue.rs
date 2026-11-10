#[derive(Clone, Copy, Debug)]
pub struct LinkedIssue;

impl LinkedIssue {
    pub const FIELDS: crate::Field<Self, crate::Object<super::Fields>> =
        crate::Field::new("fields");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
