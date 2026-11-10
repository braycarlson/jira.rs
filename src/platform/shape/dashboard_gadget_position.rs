#[derive(Clone, Copy, Debug)]
pub struct DashboardGadgetPosition;

impl DashboardGadgetPosition {
    pub const COLUMN: crate::Field<Self, crate::Integer> = crate::Field::new("column");
    pub const ROW: crate::Field<Self, crate::Integer> = crate::Field::new("row");
}
