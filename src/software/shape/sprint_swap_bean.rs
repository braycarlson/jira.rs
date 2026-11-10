#[derive(Clone, Copy, Debug)]
pub struct SprintSwapBean;

impl SprintSwapBean {
    pub const SPRINT_TO_SWAP_WITH: crate::Field<Self, crate::Integer> =
        crate::Field::new("sprintToSwapWith");
}
