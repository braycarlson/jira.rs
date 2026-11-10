#[derive(Clone, Copy, Debug)]
pub struct FeatureBean;

impl FeatureBean {
    pub const BOARD_FEATURE: crate::Field<Self, crate::Text> = crate::Field::new("boardFeature");
    pub const BOARD_ID: crate::Field<Self, crate::Integer> = crate::Field::new("boardId");
    pub const FEATURE_ID: crate::Field<Self, crate::Text> = crate::Field::new("featureId");
    pub const FEATURE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("featureType");
    pub const IMAGE_URI: crate::Field<Self, crate::Text> = crate::Field::new("imageUri");

    pub const LEARN_MORE_ARTICLE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("learnMoreArticleId");

    pub const LEARN_MORE_LINK: crate::Field<Self, crate::Text> = crate::Field::new("learnMoreLink");

    pub const LOCALISED_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("localisedDescription");

    pub const LOCALISED_GROUP: crate::Field<Self, crate::Text> =
        crate::Field::new("localisedGroup");

    pub const LOCALISED_NAME: crate::Field<Self, crate::Text> = crate::Field::new("localisedName");

    pub const PERMISSIBLE_ESTIMATION_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::FeatureBeanPermissibleEstimationTypes>>,
    > = crate::Field::new("permissibleEstimationTypes");

    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
    pub const TOGGLE_LOCKED: crate::Field<Self, crate::Boolean> = crate::Field::new("toggleLocked");
}
