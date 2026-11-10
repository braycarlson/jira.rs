#[derive(Clone, Copy, Debug)]
pub struct CreatedIssue;

impl CreatedIssue {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const TRANSITION: crate::Field<Self, crate::Object<super::NestedResponse>> =
        crate::Field::new("transition");

    pub const WATCHERS: crate::Field<Self, crate::Object<super::NestedResponse>> =
        crate::Field::new("watchers");
}
