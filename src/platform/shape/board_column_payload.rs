#[derive(Clone, Copy, Debug)]
pub struct BoardColumnPayload;

impl BoardColumnPayload {
    pub const ISSUE_CONSTRAINT_MAXIMUM: crate::Field<Self, crate::Integer> =
        crate::Field::new("maximumIssueConstraint");

    pub const ISSUE_CONSTRAINT_MINIMUM: crate::Field<Self, crate::Integer> =
        crate::Field::new("minimumIssueConstraint");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const STATUS_IDS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("statusIds");
}
