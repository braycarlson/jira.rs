#[derive(Clone, Copy, Debug)]
pub struct ToLayoutPayload;

impl ToLayoutPayload {
    pub const PORT: crate::Field<Self, crate::Integer> = crate::Field::new("port");

    pub const STATUS: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("status");
}
