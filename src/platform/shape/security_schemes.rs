#[derive(Clone, Copy, Debug)]
pub struct SecuritySchemes;

impl SecuritySchemes {
    pub const ISSUE_SECURITY_SCHEMES: crate::Field<
        Self,
        crate::List<crate::Object<super::SecurityScheme>>,
    > = crate::Field::new("issueSecuritySchemes");
}
