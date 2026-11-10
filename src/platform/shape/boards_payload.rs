#[derive(Clone, Copy, Debug)]
pub struct BoardsPayload;

impl BoardsPayload {
    pub const BOARDS: crate::Field<Self, crate::List<crate::Object<super::BoardPayload>>> =
        crate::Field::new("boards");
}
