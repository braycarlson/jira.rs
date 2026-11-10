#[derive(Clone, Copy, Debug)]
pub struct RoleActor;

impl RoleActor {
    pub const ACTOR_GROUP: crate::Field<Self, crate::Object<super::ProjectRoleGroup>> =
        crate::Field::new("actorGroup");

    pub const ACTOR_USER: crate::Field<Self, crate::Object<super::ProjectRoleUser>> =
        crate::Field::new("actorUser");

    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USER: crate::Field<Self, crate::Text> = crate::Field::new("user");
}
