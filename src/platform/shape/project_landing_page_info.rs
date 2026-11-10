#[derive(Clone, Copy, Debug)]
pub struct ProjectLandingPageInfo;

impl ProjectLandingPageInfo {
    pub const ATTRIBUTES: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("attributes");

    pub const BOARD_ID: crate::Field<Self, crate::Integer> = crate::Field::new("boardId");
    pub const BOARD_NAME: crate::Field<Self, crate::Text> = crate::Field::new("boardName");
    pub const PROJECT_KEY: crate::Field<Self, crate::Text> = crate::Field::new("projectKey");
    pub const PROJECT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("projectType");
    pub const QUEUE_CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("queueCategory");
    pub const QUEUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("queueId");
    pub const QUEUE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("queueName");
    pub const SIMPLE_BOARD: crate::Field<Self, crate::Boolean> = crate::Field::new("simpleBoard");
    pub const SIMPLIFIED: crate::Field<Self, crate::Boolean> = crate::Field::new("simplified");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
