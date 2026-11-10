#[derive(Clone, Copy, Debug)]
pub struct JiraDateInput;

impl JiraDateInput {
    pub const FORMATTED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("formattedDate");
}
