#[derive(Clone, Copy, Debug)]
pub struct WarningCollection;

impl WarningCollection {
    pub const WARNINGS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("warnings");
}
