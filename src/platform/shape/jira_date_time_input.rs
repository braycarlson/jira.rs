#[derive(Clone, Copy, Debug)]
pub struct JiraDateTimeInput;

impl JiraDateTimeInput {
    pub const FORMATTED_DATE_TIME: crate::Field<Self, crate::Text> =
        crate::Field::new("formattedDateTime");
}
