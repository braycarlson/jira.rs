#[derive(Clone, Copy, Debug)]
pub struct ScreenScheme;

impl ScreenScheme {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const ISSUE_TYPE_SCREEN_SCHEMES: crate::Field<
        Self,
        crate::Object<super::PageBeanIssueTypeScreenScheme>,
    > = crate::Field::new("issueTypeScreenSchemes");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCREENS: crate::Field<Self, crate::Object<super::ScreenTypes>> =
        crate::Field::new("screens");
}
