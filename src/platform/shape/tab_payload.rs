#[derive(Clone, Copy, Debug)]
pub struct TabPayload;

impl TabPayload {
    pub const FIELDS: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectCreateResourceIdentifier>>,
    > = crate::Field::new("fields");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
