#[derive(Clone, Copy, Debug)]
pub struct EpicUpdateBean;

impl EpicUpdateBean {
    pub const COLOR: crate::Field<Self, crate::Object<super::EpicUpdateBeanColor>> =
        crate::Field::new("color");

    pub const DONE: crate::Field<Self, crate::Boolean> = crate::Field::new("done");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
}
