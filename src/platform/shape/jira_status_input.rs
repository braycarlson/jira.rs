#[derive(Clone, Copy, Debug)]
pub struct JiraStatusInput;

impl JiraStatusInput {
    pub const STATUS_ID: crate::Field<Self, crate::Text> = crate::Field::new("statusId");
}
