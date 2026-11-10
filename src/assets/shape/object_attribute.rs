#[derive(Clone, Copy, Debug)]
pub struct ObjectAttribute;

impl ObjectAttribute {
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const OBJECT_ATTRIBUTE_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::ObjectAttributeValue>>,
    > = crate::Field::new("objectAttributeValues");

    pub const OBJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectId");

    pub const OBJECT_TYPE_ATTRIBUTE: crate::Field<Self, crate::Object<super::ObjectTypeAttribute>> =
        crate::Field::new("objectTypeAttribute");

    pub const OBJECT_TYPE_ATTRIBUTE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectTypeAttributeId");

    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
