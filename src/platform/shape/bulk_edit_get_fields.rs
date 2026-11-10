#[derive(Clone, Copy, Debug)]
pub struct BulkEditGetFields;

impl BulkEditGetFields {
    pub const ENDING_BEFORE: crate::Field<Self, crate::Text> = crate::Field::new("endingBefore");

    pub const FIELDS: crate::Field<Self, crate::List<crate::Object<super::IssueBulkEditField>>> =
        crate::Field::new("fields");

    pub const STARTING_AFTER: crate::Field<Self, crate::Text> = crate::Field::new("startingAfter");
}
