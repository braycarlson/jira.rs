#[derive(Clone, Copy, Debug)]
pub struct DataClassificationTagBean;

impl DataClassificationTagBean {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const GUIDELINE: crate::Field<Self, crate::Text> = crate::Field::new("guideline");
    pub const GUIDELINE_ADF: crate::Field<Self, crate::Text> = crate::Field::new("guidelineADF");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const RANK: crate::Field<Self, crate::Integer> = crate::Field::new("rank");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
