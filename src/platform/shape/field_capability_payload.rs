#[derive(Clone, Copy, Debug)]
pub struct FieldCapabilityPayload;

impl FieldCapabilityPayload {
    pub const CUSTOM_FIELD_DEFINITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldPayload>>,
    > = crate::Field::new("customFieldDefinitions");

    pub const FIELD_LAYOUTS: crate::Field<
        Self,
        crate::List<crate::Object<super::FieldLayoutPayload>>,
    > = crate::Field::new("fieldLayouts");

    pub const FIELD_LAYOUT_SCHEME: crate::Field<
        Self,
        crate::Object<super::FieldLayoutSchemePayload>,
    > = crate::Field::new("fieldLayoutScheme");

    pub const FIELD_SCHEME: crate::Field<Self, crate::Object<super::FieldSchemePayload>> =
        crate::Field::new("fieldScheme");

    pub const ISSUE_LAYOUTS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueLayoutPayload>>,
    > = crate::Field::new("issueLayouts");

    pub const ISSUE_TYPE_SCREEN_SCHEME: crate::Field<
        Self,
        crate::Object<super::IssueTypeScreenSchemePayload>,
    > = crate::Field::new("issueTypeScreenScheme");

    pub const SCREENS: crate::Field<Self, crate::List<crate::Object<super::ScreenPayload>>> =
        crate::Field::new("screens");

    pub const SCREEN_SCHEME: crate::Field<
        Self,
        crate::List<crate::Object<super::ScreenSchemePayload>>,
    > = crate::Field::new("screenScheme");
}
