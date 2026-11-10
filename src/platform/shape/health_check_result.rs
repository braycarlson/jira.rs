#[derive(Clone, Copy, Debug)]
pub struct HealthCheckResult;

impl HealthCheckResult {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PASSED: crate::Field<Self, crate::Boolean> = crate::Field::new("passed");
}
