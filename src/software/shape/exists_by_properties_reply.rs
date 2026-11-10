#[derive(Clone, Copy, Debug)]
pub struct ExistsByPropertiesReply;

impl ExistsByPropertiesReply {
    pub const HAS_DATA_MATCHING_PROPERTIES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasDataMatchingProperties");
}
