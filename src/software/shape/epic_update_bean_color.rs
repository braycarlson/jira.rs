#[derive(Clone, Copy, Debug)]
pub struct EpicUpdateBeanColor;

impl EpicUpdateBeanColor {
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
}
