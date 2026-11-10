#[derive(Clone, Copy, Debug)]
pub struct FieldAssociationItemPayload;

impl FieldAssociationItemPayload {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const QUALIFIER_ID: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("qualifierId");

    pub const QUALIFIER_TYPE: crate::Field<
        Self,
        crate::Object<super::ProjectCreateResourceIdentifier>,
    > = crate::Field::new("qualifierType");

    pub const RENDERER_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("rendererType");
    pub const REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("required");
}
