#[derive(Clone, Copy, Debug)]
pub struct AddSecuritySchemeLevelsRequestBean;

impl AddSecuritySchemeLevelsRequestBean {
    pub const LEVELS: crate::Field<
        Self,
        crate::List<crate::Object<super::SecuritySchemeLevelBean>>,
    > = crate::Field::new("levels");
}
