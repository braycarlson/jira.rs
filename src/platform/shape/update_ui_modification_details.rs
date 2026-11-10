#[derive(Clone, Copy, Debug)]
pub struct UpdateUIModificationDetails;

impl UpdateUIModificationDetails {
    pub const CONTEXTS: crate::Field<
        Self,
        crate::List<crate::Object<super::UIModificationContextDetails>>,
    > = crate::Field::new("contexts");

    pub const DATA: crate::Field<Self, crate::Text> = crate::Field::new("data");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
