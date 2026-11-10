#[derive(Clone, Copy, Debug)]
pub struct RequestTypeFieldDTO;

impl RequestTypeFieldDTO {
    pub const DEFAULT_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::RequestTypeFieldValueDTO>>,
    > = crate::Field::new("defaultValues");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const JIRA_SCHEMA: crate::Field<Self, crate::Object<super::JSONTypeBean>> =
        crate::Field::new("jiraSchema");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PRESET_VALUES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("presetValues");

    pub const REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("required");

    pub const VALID_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::RequestTypeFieldValueDTO>>,
    > = crate::Field::new("validValues");

    pub const VISIBLE: crate::Field<Self, crate::Boolean> = crate::Field::new("visible");
}
