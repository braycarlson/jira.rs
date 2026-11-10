#[derive(Clone, Copy, Debug)]
pub struct JiraLabelPropertiesInputJackson1;

impl JiraLabelPropertiesInputJackson1 {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
