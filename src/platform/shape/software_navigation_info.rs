#[derive(Clone, Copy, Debug)]
pub struct SoftwareNavigationInfo;

impl SoftwareNavigationInfo {
    pub const BOARDS_IN_PROJECT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalBoardsInProject");

    pub const BOARD_ID: crate::Field<Self, crate::Integer> = crate::Field::new("boardId");
    pub const BOARD_NAME: crate::Field<Self, crate::Text> = crate::Field::new("boardName");
    pub const SIMPLE_BOARD: crate::Field<Self, crate::Boolean> = crate::Field::new("simpleBoard");
}
