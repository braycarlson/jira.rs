#[derive(Clone, Copy, Debug)]
pub struct BoardCreateBean;

impl BoardCreateBean {
    pub const FILTER_ID: crate::Field<Self, crate::Integer> = crate::Field::new("filterId");

    pub const LOCATION: crate::Field<Self, crate::Object<super::BoardCreateBeanLocation>> =
        crate::Field::new("location");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
