#[derive(Clone, Copy, Debug)]
pub struct Application;

impl Application {
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
