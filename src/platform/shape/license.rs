#[derive(Clone, Copy, Debug)]
pub struct License;

impl License {
    pub const APPLICATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::LicensedApplication>>,
    > = crate::Field::new("applications");
}
