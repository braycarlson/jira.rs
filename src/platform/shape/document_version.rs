#[derive(Clone, Copy, Debug)]
pub struct DocumentVersion;

impl DocumentVersion {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const VERSION_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("versionNumber");
}
