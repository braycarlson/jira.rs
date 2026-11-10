#[derive(Clone, Copy, Debug)]
pub struct ParameterRemovalDetails;

impl ParameterRemovalDetails {
    pub const PARAMETERS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("parameters");

    pub const SCHEME_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemeId");

    pub const WORK_TYPE_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("workTypeIds");
}
