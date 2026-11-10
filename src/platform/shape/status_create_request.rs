#[derive(Clone, Copy, Debug)]
pub struct StatusCreateRequest;

impl StatusCreateRequest {
    pub const SCOPE: crate::Field<Self, crate::Object<super::StatusScope>> =
        crate::Field::new("scope");

    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::StatusCreate>>> =
        crate::Field::new("statuses");
}
