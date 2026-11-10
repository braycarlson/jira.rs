#[derive(Clone, Copy, Debug)]
pub struct RemoteObject;

impl RemoteObject {
    pub const ICON: crate::Field<Self, crate::Object<super::Icon>> = crate::Field::new("icon");

    pub const STATUS: crate::Field<Self, crate::Object<super::Status>> =
        crate::Field::new("status");

    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
