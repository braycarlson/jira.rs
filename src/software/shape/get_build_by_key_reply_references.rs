#[derive(Clone, Copy, Debug)]
pub struct GetBuildByKeyReplyReferences;

impl GetBuildByKeyReplyReferences {
    pub const COMMIT: crate::Field<Self, crate::Object<super::GetBuildByKeyReplyReferencesCommit>> =
        crate::Field::new("commit");

    pub const REFERENCE: crate::Field<Self, crate::Object<super::GetBuildByKeyReplyReferencesRef>> =
        crate::Field::new("ref");
}
