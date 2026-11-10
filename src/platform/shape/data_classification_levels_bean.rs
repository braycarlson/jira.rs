#[derive(Clone, Copy, Debug)]
pub struct DataClassificationLevelsBean;

impl DataClassificationLevelsBean {
    pub const CLASSIFICATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::DataClassificationTagBean>>,
    > = crate::Field::new("classifications");
}
