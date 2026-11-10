#[derive(Clone, Copy, Debug)]
pub struct ForgePanelProjectPinStatus;

impl ForgePanelProjectPinStatus {
    pub const ERROR: crate::Field<Self, crate::Text> = crate::Field::new("error");
    pub const PINNED: crate::Field<Self, crate::Boolean> = crate::Field::new("pinned");
    pub const PINNED_AT: crate::Field<Self, crate::Integer> = crate::Field::new("pinnedAt");

    pub const PROJECT_ID_OR_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectIdOrKey");
}
