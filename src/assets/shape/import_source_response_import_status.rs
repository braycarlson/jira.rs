#[derive(Clone, Copy, Debug)]
pub struct ImportSourceResponseImportStatus;

impl ImportSourceResponseImportStatus {
    pub const CONFIGURATION_AUI_LOZENGE: crate::Field<Self, crate::Text> =
        crate::Field::new("configurationAuiLozenge");

    pub const CONFIGURATION_STATUS_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("configurationStatusType");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const REASON_FOR_INVALIDITY: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("reasonForInvalidity");

    pub const VALIDATION: crate::Field<Self, crate::Text> = crate::Field::new("validation");

    pub const VALIDATION_AUI_LOZENGE: crate::Field<Self, crate::Text> =
        crate::Field::new("validationAuiLozenge");

    pub const VALIDATION_STATUS_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("validationStatusType");
}
