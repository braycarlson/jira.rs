#[derive(Clone, Copy, Debug)]
pub struct StatusProjectUsageDTO;

impl StatusProjectUsageDTO {
    pub const PROJECTS: crate::Field<Self, crate::Object<super::StatusProjectUsagePage>> =
        crate::Field::new("projects");

    pub const STATUS_ID: crate::Field<Self, crate::Text> = crate::Field::new("statusId");
}
