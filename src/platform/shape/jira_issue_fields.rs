#[derive(Clone, Copy, Debug)]
pub struct JiraIssueFields;

impl JiraIssueFields {
    pub const CASCADING_SELECT_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraCascadingSelectField>>,
    > = crate::Field::new("cascadingSelectFields");

    pub const CLEARABLE_NUMBER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraNumberField>>,
    > = crate::Field::new("clearableNumberFields");

    pub const COLOR_FIELDS: crate::Field<Self, crate::List<crate::Object<super::JiraColorField>>> =
        crate::Field::new("colorFields");

    pub const DATE_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraDateField>>,
    > = crate::Field::new("datePickerFields");

    pub const DATE_TIME_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraDateTimeField>>,
    > = crate::Field::new("dateTimePickerFields");

    pub const ISSUE_TYPE: crate::Field<Self, crate::Object<super::JiraIssueTypeField>> =
        crate::Field::new("issueType");

    pub const LABELS_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraLabelsField>>,
    > = crate::Field::new("labelsFields");

    pub const MULTIPLE_GROUP_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraMultipleGroupPickerField>>,
    > = crate::Field::new("multipleGroupPickerFields");

    pub const MULTIPLE_SELECT_CLEARABLE_USER_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraMultipleSelectUserPickerField>>,
    > = crate::Field::new("multipleSelectClearableUserPickerFields");

    pub const MULTIPLE_SELECT_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraMultipleSelectField>>,
    > = crate::Field::new("multipleSelectFields");

    pub const MULTIPLE_VERSION_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraMultipleVersionPickerField>>,
    > = crate::Field::new("multipleVersionPickerFields");

    pub const MULTISELECT_COMPONENTS: crate::Field<
        Self,
        crate::Object<super::JiraMultiSelectComponentField>,
    > = crate::Field::new("multiselectComponents");

    pub const ORIGINAL_ESTIMATE_FIELD: crate::Field<Self, crate::Object<super::JiraDurationField>> =
        crate::Field::new("originalEstimateField");

    pub const PRIORITY: crate::Field<Self, crate::Object<super::JiraPriorityField>> =
        crate::Field::new("priority");

    pub const RICH_TEXT_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraRichTextField>>,
    > = crate::Field::new("richTextFields");

    pub const SINGLE_GROUP_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraSingleGroupPickerField>>,
    > = crate::Field::new("singleGroupPickerFields");

    pub const SINGLE_LINE_TEXT_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraSingleLineTextField>>,
    > = crate::Field::new("singleLineTextFields");

    pub const SINGLE_SELECT_CLEARABLE_USER_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraSingleSelectUserPickerField>>,
    > = crate::Field::new("singleSelectClearableUserPickerFields");

    pub const SINGLE_SELECT_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraSingleSelectField>>,
    > = crate::Field::new("singleSelectFields");

    pub const SINGLE_VERSION_PICKER_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraSingleVersionPickerField>>,
    > = crate::Field::new("singleVersionPickerFields");

    pub const STATUS: crate::Field<Self, crate::Object<super::JiraStatusInput>> =
        crate::Field::new("status");

    pub const TIME_TRACKING_FIELD: crate::Field<Self, crate::Object<super::JiraTimeTrackingField>> =
        crate::Field::new("timeTrackingField");

    pub const URL_FIELDS: crate::Field<Self, crate::List<crate::Object<super::JiraURLField>>> =
        crate::Field::new("urlFields");
}
