#[derive(Clone, Copy, Debug)]
pub struct JiraTimeTrackingField;

impl JiraTimeTrackingField {
    pub const TIME_REMAINING: crate::Field<Self, crate::Text> = crate::Field::new("timeRemaining");
}
