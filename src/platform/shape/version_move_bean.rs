#[derive(Clone, Copy, Debug)]
pub struct VersionMoveBean;

impl VersionMoveBean {
    pub const AFTER: crate::Field<Self, crate::Text> = crate::Field::new("after");
    pub const POSITION: crate::Field<Self, crate::Text> = crate::Field::new("position");
}
