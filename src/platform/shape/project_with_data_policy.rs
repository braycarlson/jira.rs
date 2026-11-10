#[derive(Clone, Copy, Debug)]
pub struct ProjectWithDataPolicy;

impl ProjectWithDataPolicy {
    pub const DATA_POLICY: crate::Field<Self, crate::Object<super::ProjectDataPolicy>> =
        crate::Field::new("dataPolicy");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
}
