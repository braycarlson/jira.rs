#[derive(Clone, Copy, Debug)]
pub struct CardLayout;

impl CardLayout {
    pub const SHOW_DAYS_IN_COLUMN: crate::Field<Self, crate::Boolean> =
        crate::Field::new("showDaysInColumn");
}
