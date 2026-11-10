#[derive(Clone, Copy, Debug)]
pub struct WorkflowStatusLayout;

impl WorkflowStatusLayout {
    pub const X_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("x");
    pub const Y_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("y");
}
