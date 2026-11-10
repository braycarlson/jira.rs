#[derive(Clone, Copy, Debug)]
pub struct ProjectRoleActorsUpdateBean;

impl ProjectRoleActorsUpdateBean {
    pub const CATEGORISED_ACTORS: crate::Field<Self, crate::Map<crate::List<crate::Text>>> =
        crate::Field::new("categorisedActors");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
}
