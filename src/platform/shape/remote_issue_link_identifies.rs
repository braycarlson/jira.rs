#[derive(Clone, Copy, Debug)]
pub struct RemoteIssueLinkIdentifies;

impl RemoteIssueLinkIdentifies {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
