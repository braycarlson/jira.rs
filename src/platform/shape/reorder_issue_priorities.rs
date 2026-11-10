#[derive(Clone, Copy, Debug)]
pub struct ReorderIssuePriorities;

impl ReorderIssuePriorities {
    pub const AFTER: crate::Field<Self, crate::Text> = crate::Field::new("after");
    pub const IDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("ids");
    pub const POSITION: crate::Field<Self, crate::Text> = crate::Field::new("position");
}
