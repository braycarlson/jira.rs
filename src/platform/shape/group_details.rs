#[derive(Clone, Copy, Debug)]
pub struct GroupDetails;

impl GroupDetails {
    pub const GROUP_ID: crate::Field<Self, crate::Text> = crate::Field::new("groupId");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
