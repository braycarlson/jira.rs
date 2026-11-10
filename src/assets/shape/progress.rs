#[derive(Clone, Copy, Debug)]
pub struct Progress;

impl Progress {
    pub const ACTOR: crate::Field<Self, crate::Text> = crate::Field::new("actor");
    pub const CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("category");

    pub const ESTIMATED_FINISH_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("estimatedFinishDate");

    pub const EXECUTION_UUID: crate::Field<Self, crate::Text> = crate::Field::new("executionUUID");
    pub const FINISHED_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("finishedDate");

    pub const NUMBER_OF_STEPS: crate::Field<Self, crate::Integer> =
        crate::Field::new("numberOfSteps");

    pub const PROGRESS_IN_PERCENT: crate::Field<Self, crate::Integer> =
        crate::Field::new("progressInPercent");

    pub const RESOURCE_ID: crate::Field<Self, crate::Text> = crate::Field::new("resourceId");
    pub const RESULT: crate::Field<Self, crate::Text> = crate::Field::new("result");
    pub const RESULT_DATA: crate::Field<Self, crate::Any> = crate::Field::new("resultData");
    pub const RESULT_MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("resultMessage");
    pub const START_DATE: crate::Field<Self, crate::Integer> = crate::Field::new("startDate");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const STEP_CURRENT: crate::Field<Self, crate::Integer> = crate::Field::new("currentStep");

    pub const STEP_DESCRIPTION: crate::Field<Self, crate::Text> =
        crate::Field::new("stepDescription");

    pub const STEP_WORK_UNITS_CURRENT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("currentStepTotalWorkUnits");

    pub const WORK_DESCRIPTION_CURRENT: crate::Field<Self, crate::Text> =
        crate::Field::new("currentWorkDescription");

    pub const WORK_UNITS_CURRENT: crate::Field<Self, crate::Integer> =
        crate::Field::new("currentWorkUnits");

    pub const WORK_UNITS_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalWorkUnits");
}
