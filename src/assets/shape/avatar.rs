#[derive(Clone, Copy, Debug)]
pub struct Avatar;

impl Avatar {
    pub const AVATAR_UUID: crate::Field<Self, crate::Text> = crate::Field::new("avatarUUID");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const MEDIA_CLIENT_CONFIG: crate::Field<Self, crate::Any> =
        crate::Field::new("mediaClientConfig");

    pub const OBJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("objectId");
    pub const URL144: crate::Field<Self, crate::Text> = crate::Field::new("url144");
    pub const URL16: crate::Field<Self, crate::Text> = crate::Field::new("url16");
    pub const URL288: crate::Field<Self, crate::Text> = crate::Field::new("url288");
    pub const URL48: crate::Field<Self, crate::Text> = crate::Field::new("url48");
    pub const URL72: crate::Field<Self, crate::Text> = crate::Field::new("url72");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
