#[derive(Clone, Copy, Debug)]
pub struct IssueEntityProperties;

impl IssueEntityProperties {
    pub const ENTITIES_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("entitiesIds");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Object<super::JSONNode>>> =
        crate::Field::new("properties");
}
