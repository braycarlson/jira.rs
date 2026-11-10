#[derive(Clone, Copy, Debug)]
pub struct FormAnswer;

impl FormAnswer {
    pub const ADF: crate::Field<Self, crate::Object<super::JSONNode>> = crate::Field::new("adf");
    pub const CHOICES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("choices");
    pub const DATE: crate::Field<Self, crate::Text> = crate::Field::new("date");
    pub const FILES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("files");
    pub const TEXT: crate::Field<Self, crate::Text> = crate::Field::new("text");
    pub const TIME: crate::Field<Self, crate::Text> = crate::Field::new("time");
    pub const USERS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("users");
}
