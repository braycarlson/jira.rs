#[derive(Clone, Copy, Debug)]
pub struct IDBean;

impl IDBean {
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
}
