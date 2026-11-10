#[derive(Clone, Copy, Debug)]
pub struct MoveIssuesToBoardReply;

impl MoveIssuesToBoardReply {
    pub const ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::MoveIssuesToBoardReplyEntries>>,
    > = crate::Field::new("entries");
}
