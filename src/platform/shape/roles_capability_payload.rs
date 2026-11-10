#[derive(Clone, Copy, Debug)]
pub struct RolesCapabilityPayload;

impl RolesCapabilityPayload {
    pub const ROLES: crate::Field<Self, crate::List<crate::Object<super::RolePayload>>> =
        crate::Field::new("roles");

    pub const ROLE_TO_PROJECT_ACTORS: crate::Field<
        Self,
        crate::Map<crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>>,
    > = crate::Field::new("roleToProjectActors");
}
