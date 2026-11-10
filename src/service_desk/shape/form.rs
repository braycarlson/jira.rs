#[derive(Clone, Copy, Debug)]
pub struct Form;

impl Form {
    pub const ANSWERS: crate::Field<Self, crate::Map<crate::Object<super::FormAnswer>>> =
        crate::Field::new("answers");
}
