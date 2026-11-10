#[derive(Clone, Copy, Debug)]
pub struct KeywordOperand;

impl KeywordOperand {
    pub const KEYWORD: crate::Field<Self, crate::Text> = crate::Field::new("keyword");
}
