#[derive(Clone, Copy, Debug)]
pub struct AttachmentSettings;

impl AttachmentSettings {
    pub const ENABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("enabled");
    pub const UPLOAD_LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("uploadLimit");
}
