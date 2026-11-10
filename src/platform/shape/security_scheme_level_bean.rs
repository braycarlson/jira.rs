#[derive(Clone, Copy, Debug)]
pub struct SecuritySchemeLevelBean;

impl SecuritySchemeLevelBean {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");

    pub const MEMBERS: crate::Field<
        Self,
        crate::List<crate::Object<super::SecuritySchemeLevelMemberBean>>,
    > = crate::Field::new("members");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
