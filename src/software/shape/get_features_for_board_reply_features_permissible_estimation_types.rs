#[derive(Clone, Copy, Debug)]
pub struct GetFeaturesForBoardReplyFeaturesPermissibleEstimationTypes;

impl GetFeaturesForBoardReplyFeaturesPermissibleEstimationTypes {
    pub const LOCALISED_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("localisedDescription");

    pub const LOCALISED_NAME: crate::Field<Self, crate::Text> = crate::Field::new("localisedName");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
