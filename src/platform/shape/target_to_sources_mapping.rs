#[derive(Clone, Copy, Debug)]
pub struct TargetToSourcesMapping;

impl TargetToSourcesMapping {
    pub const INFER_CLASSIFICATION_DEFAULTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("inferClassificationDefaults");

    pub const INFER_FIELD_DEFAULTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("inferFieldDefaults");

    pub const INFER_STATUS_DEFAULTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("inferStatusDefaults");

    pub const INFER_SUBTASK_TYPE_DEFAULT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("inferSubtaskTypeDefault");

    pub const ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIdsOrKeys");

    pub const TARGET_CLASSIFICATION: crate::Field<
        Self,
        crate::List<crate::Object<super::TargetClassification>>,
    > = crate::Field::new("targetClassification");

    pub const TARGET_MANDATORY_FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::TargetMandatoryFields>>,
    > = crate::Field::new("targetMandatoryFields");

    pub const TARGET_STATUS: crate::Field<Self, crate::List<crate::Object<super::TargetStatus>>> =
        crate::Field::new("targetStatus");
}
