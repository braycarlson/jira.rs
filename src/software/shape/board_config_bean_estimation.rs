#[derive(Clone, Copy, Debug)]
pub struct BoardConfigBeanEstimation;

impl BoardConfigBeanEstimation {
    pub const FIELD: crate::Field<Self, crate::Object<super::BoardConfigBeanEstimationField>> =
        crate::Field::new("field");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
