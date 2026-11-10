#[derive(Clone, Copy, Debug)]
pub struct WorkflowLayout;

impl WorkflowLayout {
    pub const X_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("x");
    pub const Y_VALUE: crate::Field<Self, crate::Number> = crate::Field::new("y");
}
