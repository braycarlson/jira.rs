#[derive(Clone, Copy, Debug)]
pub struct WorkManagementNavigationInfo;

impl WorkManagementNavigationInfo {
    pub const BOARD_NAME: crate::Field<Self, crate::Text> = crate::Field::new("boardName");
}
