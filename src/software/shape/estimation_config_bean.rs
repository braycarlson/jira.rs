#[derive(Clone, Copy, Debug)]
pub struct EstimationConfigBean;

impl EstimationConfigBean {
    pub const FIELD: crate::Field<Self, crate::Object<super::EstimationConfigBeanField>> =
        crate::Field::new("field");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
