#[derive(Clone, Copy, Debug)]
pub struct RolePayload;

impl RolePayload {
    pub const DEFAULT_ACTORS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("defaultActors");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
