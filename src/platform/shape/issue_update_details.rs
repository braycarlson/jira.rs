#[derive(Clone, Copy, Debug)]
pub struct IssueUpdateDetails;

impl IssueUpdateDetails {
    pub const FIELDS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("fields");

    pub const HISTORY_METADATA: crate::Field<Self, crate::Object<super::HistoryMetadata>> =
        crate::Field::new("historyMetadata");

    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Object<super::EntityProperty>>> =
        crate::Field::new("properties");

    pub const TRANSITION: crate::Field<Self, crate::Object<super::IssueTransition>> =
        crate::Field::new("transition");

    pub const UPDATE: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::FieldUpdateOperation>>>,
    > = crate::Field::new("update");
}
