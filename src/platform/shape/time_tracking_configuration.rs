#[derive(Clone, Copy, Debug)]
pub struct TimeTrackingConfiguration;

impl TimeTrackingConfiguration {
    pub const DEFAULT_UNIT: crate::Field<Self, crate::Text> = crate::Field::new("defaultUnit");
    pub const TIME_FORMAT: crate::Field<Self, crate::Text> = crate::Field::new("timeFormat");

    pub const WORKING_DAYS_PER_WEEK: crate::Field<Self, crate::Number> =
        crate::Field::new("workingDaysPerWeek");

    pub const WORKING_HOURS_PER_DAY: crate::Field<Self, crate::Number> =
        crate::Field::new("workingHoursPerDay");
}
