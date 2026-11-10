#[derive(Clone, Copy, Debug)]
pub struct GetBuildByKeyReplyTestInfo;

impl GetBuildByKeyReplyTestInfo {
    pub const NUMBER_FAILED: crate::Field<Self, crate::Integer> = crate::Field::new("numberFailed");
    pub const NUMBER_PASSED: crate::Field<Self, crate::Integer> = crate::Field::new("numberPassed");

    pub const NUMBER_SKIPPED: crate::Field<Self, crate::Integer> =
        crate::Field::new("numberSkipped");

    pub const NUMBER_TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("totalNumber");
}
