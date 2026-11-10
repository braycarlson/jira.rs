#[derive(Clone, Copy, Debug)]
pub struct HistoryMetadata;

impl HistoryMetadata {
    pub const ACTIVITY_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("activityDescription");

    pub const ACTIVITY_DESCRIPTION_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("activityDescriptionKey");

    pub const ACTOR: crate::Field<Self, crate::Object<super::HistoryMetadataActor>> =
        crate::Field::new("actor");

    pub const CAUSE: crate::Field<Self, crate::Object<super::HistoryMetadataCause>> =
        crate::Field::new("cause");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const DESCRIPTION_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("descriptionKey");

    pub const EMAIL_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("emailDescription");

    pub const EMAIL_DESCRIPTION_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("emailDescriptionKey");

    pub const EXTRA_DATA: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("extraData");

    pub const GENERATOR: crate::Field<Self, crate::Object<super::HistoryMetadataGenerator>> =
        crate::Field::new("generator");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
