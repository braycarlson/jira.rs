#[derive(Clone, Copy, Debug)]
pub struct MultiIssueEntityProperties;

impl MultiIssueEntityProperties {
    pub const ISSUES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueEntityPropertiesForMultiUpdate>>,
    > = crate::Field::new("issues");
}
