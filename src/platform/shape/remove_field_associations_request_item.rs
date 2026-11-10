#[derive(Clone, Copy, Debug)]
pub struct RemoveFieldAssociationsRequestItem;

impl RemoveFieldAssociationsRequestItem {
    pub const SCHEME_IDS: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("schemeIds");
}
