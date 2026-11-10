#[derive(Clone, Copy, Debug)]
pub struct StatusMigration;

impl StatusMigration {
    pub const STATUS_REFERENCE_NEW: crate::Field<Self, crate::Text> =
        crate::Field::new("newStatusReference");

    pub const STATUS_REFERENCE_OLD: crate::Field<Self, crate::Text> =
        crate::Field::new("oldStatusReference");
}
