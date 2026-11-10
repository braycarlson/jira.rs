#[derive(Clone, Copy, Debug)]
pub struct FromLayoutPayload;

impl FromLayoutPayload {
    pub const FROM_PORT: crate::Field<Self, crate::Integer> = crate::Field::new("fromPort");

    pub const STATUS: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("status");

    pub const TO_PORT_OVERRIDE: crate::Field<Self, crate::Integer> =
        crate::Field::new("toPortOverride");
}
