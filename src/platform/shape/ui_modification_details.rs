#[derive(Clone, Copy, Debug)]
pub struct UIModificationDetails;

impl UIModificationDetails {
    pub const CONTEXTS: crate::Field<
        Self,
        crate::List<crate::Object<super::UIModificationContextDetails>>,
    > = crate::Field::new("contexts");

    pub const DATA: crate::Field<Self, crate::Text> = crate::Field::new("data");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
