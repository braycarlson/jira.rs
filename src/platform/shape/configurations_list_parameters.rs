#[derive(Clone, Copy, Debug)]
pub struct ConfigurationsListParameters;

impl ConfigurationsListParameters {
    pub const FIELD_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("fieldIdsOrKeys");
}
