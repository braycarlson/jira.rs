#[derive(Clone, Copy, Debug)]
pub struct ObjectHistory;

impl ObjectHistory {
    pub const ACTOR: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("actor");

    pub const AFFECTED_ATTRIBUTE: crate::Field<Self, crate::Text> =
        crate::Field::new("affectedAttribute");

    pub const AFFECTED_ATTRIBUTE_DISPLAY_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("affectedAttributeDisplayName");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const DISPLAY_VALUES: crate::Field<Self, crate::Any> = crate::Field::new("displayValues");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const OBJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectId");
    pub const TYPE: crate::Field<Self, crate::Integer> = crate::Field::new("type");
    pub const VALUE_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newValue");
    pub const VALUE_OLD: crate::Field<Self, crate::Text> = crate::Field::new("oldValue");
}
