#[derive(Clone, Copy, Debug)]
pub struct MoveFieldBean;

impl MoveFieldBean {
    pub const AFTER: crate::Field<Self, crate::Text> = crate::Field::new("after");
    pub const POSITION: crate::Field<Self, crate::Text> = crate::Field::new("position");
}
