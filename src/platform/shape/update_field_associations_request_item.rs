#[derive(Clone, Copy, Debug)]
pub struct UpdateFieldAssociationsRequestItem;

impl UpdateFieldAssociationsRequestItem {
    pub const RESTRICTED_TO_WORK_TYPES: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("restrictedToWorkTypes");

    pub const SCHEME_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("schemeIds");
}
