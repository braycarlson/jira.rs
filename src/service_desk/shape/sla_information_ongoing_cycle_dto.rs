#[derive(Clone, Copy, Debug)]
pub struct SlaInformationOngoingCycleDTO;

impl SlaInformationOngoingCycleDTO {
    pub const BREACHED: crate::Field<Self, crate::Boolean> = crate::Field::new("breached");

    pub const BREACH_TIME: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("breachTime");

    pub const ELAPSED_TIME: crate::Field<Self, crate::Object<super::DurationDTO>> =
        crate::Field::new("elapsedTime");

    pub const GOAL_DURATION: crate::Field<Self, crate::Object<super::DurationDTO>> =
        crate::Field::new("goalDuration");

    pub const PAUSED: crate::Field<Self, crate::Boolean> = crate::Field::new("paused");

    pub const REMAINING_TIME: crate::Field<Self, crate::Object<super::DurationDTO>> =
        crate::Field::new("remainingTime");

    pub const START_TIME: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("startTime");

    pub const WITHIN_CALENDAR_HOURS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("withinCalendarHours");
}
