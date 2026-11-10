#[derive(Clone, Copy, Debug)]
pub struct PrioritySchemeID;

impl PrioritySchemeID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const TASK: crate::Field<Self, crate::Object<super::TaskProgressBeanJSONNode>> =
        crate::Field::new("task");
}
