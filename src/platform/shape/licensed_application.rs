#[derive(Clone, Copy, Debug)]
pub struct LicensedApplication;

impl LicensedApplication {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const PLAN: crate::Field<Self, crate::Text> = crate::Field::new("plan");
}
