#[derive(Clone, Copy, Debug)]
pub struct ProjectFeature;

impl ProjectFeature {
    pub const FEATURE: crate::Field<Self, crate::Text> = crate::Field::new("feature");
    pub const IMAGE_URI: crate::Field<Self, crate::Text> = crate::Field::new("imageUri");

    pub const LOCALISED_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("localisedDescription");

    pub const LOCALISED_NAME: crate::Field<Self, crate::Text> = crate::Field::new("localisedName");

    pub const PREREQUISITES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("prerequisites");

    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
    pub const TOGGLE_LOCKED: crate::Field<Self, crate::Boolean> = crate::Field::new("toggleLocked");
}
