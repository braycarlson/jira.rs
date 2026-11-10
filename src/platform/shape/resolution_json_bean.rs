#[derive(Clone, Copy, Debug)]
pub struct ResolutionJSONBean;

impl ResolutionJSONBean {
    pub const DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("default");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
