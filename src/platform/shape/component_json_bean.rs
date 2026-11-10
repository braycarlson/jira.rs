#[derive(Clone, Copy, Debug)]
pub struct ComponentJSONBean;

impl ComponentJSONBean {
    pub const ARI: crate::Field<Self, crate::Text> = crate::Field::new("ari");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const METADATA: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("metadata");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
