#[derive(Clone, Copy, Debug)]
pub struct GetBuildByKeyReplyReferencesRef;

impl GetBuildByKeyReplyReferencesRef {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const URI: crate::Field<Self, crate::Text> = crate::Field::new("uri");
}
