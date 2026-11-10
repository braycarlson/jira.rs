#[derive(Clone, Copy, Debug)]
pub struct PriorityWithSequence;

impl PriorityWithSequence {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ICON_URL: crate::Field<Self, crate::Text> = crate::Field::new("iconUrl");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const SEQUENCE: crate::Field<Self, crate::Text> = crate::Field::new("sequence");
    pub const STATUS_COLOR: crate::Field<Self, crate::Text> = crate::Field::new("statusColor");
}
