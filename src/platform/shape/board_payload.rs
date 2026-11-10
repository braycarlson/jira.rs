#[derive(Clone, Copy, Debug)]
pub struct BoardPayload;

impl BoardPayload {
    pub const BOARD_FILTER_JQL: crate::Field<Self, crate::Text> =
        crate::Field::new("boardFilterJQL");

    pub const CARD_COLOR_STRATEGY: crate::Field<Self, crate::Text> =
        crate::Field::new("cardColorStrategy");

    pub const CARD_LAYOUT: crate::Field<Self, crate::Object<super::CardLayout>> =
        crate::Field::new("cardLayout");

    pub const CARD_LAYOUTS: crate::Field<Self, crate::List<crate::Object<super::CardLayoutField>>> =
        crate::Field::new("cardLayouts");

    pub const COLUMNS: crate::Field<Self, crate::List<crate::Object<super::BoardColumnPayload>>> =
        crate::Field::new("columns");

    pub const ENABLE_CARD_COVER: crate::Field<Self, crate::Boolean> =
        crate::Field::new("enableCardCover");

    pub const FEATURES: crate::Field<Self, crate::List<crate::Object<super::BoardFeaturePayload>>> =
        crate::Field::new("features");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const QUICK_FILTERS: crate::Field<
        Self,
        crate::List<crate::Object<super::QuickFilterPayload>>,
    > = crate::Field::new("quickFilters");

    pub const SUPPORTS_SPRINT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("supportsSprint");

    pub const SWIMLANES: crate::Field<Self, crate::Object<super::SwimlanesPayload>> =
        crate::Field::new("swimlanes");

    pub const WORKING_DAYS_CONFIG: crate::Field<Self, crate::Object<super::WorkingDaysConfig>> =
        crate::Field::new("workingDaysConfig");
}
