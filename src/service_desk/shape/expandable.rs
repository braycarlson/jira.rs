#[derive(Clone, Copy, Debug)]
pub struct Expandable;

impl Expandable {
    pub const EXPANDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("_expands");
}
