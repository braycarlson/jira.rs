#[derive(Clone, Copy, Debug)]
pub struct IssueEntityPropertiesForMultiUpdate;

impl IssueEntityPropertiesForMultiUpdate {
    pub const ISSUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("issueID");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Object<super::JSONNode>>> =
        crate::Field::new("properties");
}
