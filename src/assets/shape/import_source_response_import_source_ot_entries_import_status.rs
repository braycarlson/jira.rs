#[derive(Clone, Copy, Debug)]
pub struct ImportSourceResponseImportSourceOTEntriesImportStatus;

impl ImportSourceResponseImportSourceOTEntriesImportStatus {
    pub const CONFIGURATION_STATUS_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("configurationStatusType");

    pub const REASON_FOR_INVALIDITY: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("reasonForInvalidity");

    pub const VALIDATION_STATUS_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("validationStatusType");
}
