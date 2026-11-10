#[derive(Clone, Copy, Debug)]
pub struct ObjectTypePosition;

impl ObjectTypePosition {
    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");

    pub const TO_OBJECT_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("toObjectTypeId");
}
