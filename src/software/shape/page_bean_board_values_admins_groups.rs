#[derive(Clone, Copy, Debug)]
pub struct PageBeanBoardValuesAdminsGroups;

impl PageBeanBoardValuesAdminsGroups {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
