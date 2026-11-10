#[derive(Clone, Copy, Debug)]
pub struct RemoteIssueLinkRequest;

impl RemoteIssueLinkRequest {
    pub const APPLICATION: crate::Field<Self, crate::Object<super::Application>> =
        crate::Field::new("application");

    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");

    pub const OBJECT: crate::Field<Self, crate::Object<super::RemoteObject>> =
        crate::Field::new("object");

    pub const RELATIONSHIP: crate::Field<Self, crate::Text> = crate::Field::new("relationship");
}
