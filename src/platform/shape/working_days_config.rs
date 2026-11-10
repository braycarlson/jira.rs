#[derive(Clone, Copy, Debug)]
pub struct WorkingDaysConfig;

impl WorkingDaysConfig {
    pub const FRIDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("friday");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const MONDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("monday");

    pub const NON_WORKING_DAYS: crate::Field<
        Self,
        crate::List<crate::Object<super::NonWorkingDay>>,
    > = crate::Field::new("nonWorkingDays");

    pub const SATURDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("saturday");
    pub const SUNDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("sunday");
    pub const THURSDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("thursday");
    pub const TIMEZONE_ID: crate::Field<Self, crate::Text> = crate::Field::new("timezoneId");
    pub const TUESDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("tuesday");
    pub const WEDNESDAY: crate::Field<Self, crate::Boolean> = crate::Field::new("wednesday");
}
