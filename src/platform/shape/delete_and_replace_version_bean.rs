#[derive(Clone, Copy, Debug)]
pub struct DeleteAndReplaceVersionBean;

impl DeleteAndReplaceVersionBean {
    pub const CUSTOM_FIELD_REPLACEMENT_LIST: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomFieldReplacement>>,
    > = crate::Field::new("customFieldReplacementList");

    pub const MOVE_AFFECTED_ISSUES_TO: crate::Field<Self, crate::Integer> =
        crate::Field::new("moveAffectedIssuesTo");

    pub const MOVE_FIX_ISSUES_TO: crate::Field<Self, crate::Integer> =
        crate::Field::new("moveFixIssuesTo");
}
