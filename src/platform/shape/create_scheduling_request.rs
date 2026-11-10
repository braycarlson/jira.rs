#[derive(Clone, Copy, Debug)]
pub struct CreateSchedulingRequest;

impl CreateSchedulingRequest {
    pub const DEPENDENCIES: crate::Field<Self, crate::Text> = crate::Field::new("dependencies");

    pub const END_DATE: crate::Field<Self, crate::Object<super::CreateDateFieldRequest>> =
        crate::Field::new("endDate");

    pub const ESTIMATION: crate::Field<Self, crate::Text> = crate::Field::new("estimation");
    pub const INFERRED_DATES: crate::Field<Self, crate::Text> = crate::Field::new("inferredDates");

    pub const START_DATE: crate::Field<Self, crate::Object<super::CreateDateFieldRequest>> =
        crate::Field::new("startDate");
}
