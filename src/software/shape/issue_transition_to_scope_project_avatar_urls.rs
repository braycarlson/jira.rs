#[derive(Clone, Copy, Debug)]
pub struct IssueTransitionToScopeProjectAvatarURLs;

impl IssueTransitionToScopeProjectAvatarURLs {
    pub const VALUE_16X16: crate::Field<Self, crate::Text> = crate::Field::new("16x16");
    pub const VALUE_24X24: crate::Field<Self, crate::Text> = crate::Field::new("24x24");
    pub const VALUE_32X32: crate::Field<Self, crate::Text> = crate::Field::new("32x32");
    pub const VALUE_48X48: crate::Field<Self, crate::Text> = crate::Field::new("48x48");
}
