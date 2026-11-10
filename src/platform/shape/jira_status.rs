#[derive(Clone, Copy, Debug)]
pub struct JiraStatus;

impl JiraStatus {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCOPE: crate::Field<Self, crate::Object<super::StatusScope>> =
        crate::Field::new("scope");

    pub const STATUS_CATEGORY: crate::Field<Self, crate::Text> =
        crate::Field::new("statusCategory");
}
