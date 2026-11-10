#[derive(Clone, Copy, Debug)]
pub struct GetIncidentByIDReplySeverity;

impl GetIncidentByIDReplySeverity {
    pub const LEVEL: crate::Field<Self, crate::Text> = crate::Field::new("level");
}
