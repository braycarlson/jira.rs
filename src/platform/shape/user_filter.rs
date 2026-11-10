#[derive(Clone, Copy, Debug)]
pub struct UserFilter;

impl UserFilter {
    pub const ENABLED: crate::Field<Self, crate::Boolean> = crate::Field::new("enabled");
    pub const GROUPS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("groups");

    pub const ROLE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("roleIds");
}
