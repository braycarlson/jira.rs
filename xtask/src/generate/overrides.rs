use alloc::collections::BTreeSet;

pub(crate) const ANY_REPLIES: [&str; 3] = ["getDraftWorkflow", "getProjectsFull", "getWorkflow"];

pub(crate) const BARE_WALKS: [&str; 3] =
    ["bulkGetUsersMigration", "getAllUsers", "getAllUsersDefault"];

pub(crate) const BINARY_REPLIES: [&str; 5] = [
    "getAttachmentContent",
    "getAttachmentThumbnail",
    "getAvatarImageByID",
    "getAvatarImageByOwner",
    "getAvatarImageByType",
];

pub(crate) const BODIES_UNDECLARED: [&str; 1] = [concat!(
    "put /rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}",
    "/property/{propertyKey}",
)];

pub(crate) const FIELDS_UNDECLARED: [(&str, &str, &str); 89] = [
    ("AuditRecordBean", "authorAccountId", "crate::Text"),
    ("Avatar", "mediaClientConfig", "crate::Any"),
    ("BulkIssueResults", "expand", "crate::Text"),
    ("ChangeDetails", "tmpFromAccountId", "crate::Text"),
    ("ChangeDetails", "tmpToAccountId", "crate::Text"),
    ("ComponentWithIssueCount", "componentBean", "crate::Any"),
    ("Context", "associatedIssueTypes", "crate::Any"),
    ("Context", "associatedProjects", "crate::Any"),
    ("Context", "description", "crate::Text"),
    ("Context", "isAllIssueTypes", "crate::Boolean"),
    ("Context", "isAllProjects", "crate::Boolean"),
    ("EntityProperty", "self", "crate::Text"),
    ("Field", "areOptionsSupported", "crate::Boolean"),
    ("Field", "isOptionsCountOverLimit", "crate::Boolean"),
    ("Field", "plannedDeletionDate", "crate::Text"),
    ("Field", "trashedBy", "crate::Any"),
    ("Field", "trashedDate", "crate::Text"),
    ("FieldConfigurationDetails", "id", "crate::Integer"),
    ("FieldConfigurationDetails", "isDefault", "crate::Boolean"),
    ("FieldDetails", "untranslatedName", "crate::Text"),
    ("Filter", "isWritable", "crate::Boolean"),
    ("GetConfigurationReplyLocation", "id", "crate::Text"),
    ("GetConfigurationReplyLocation", "key", "crate::Text"),
    ("GetConfigurationReplyLocation", "name", "crate::Text"),
    ("GetConfigurationReplyLocation", "self", "crate::Text"),
    ("GetFeaturesForBoardReplyFeatures", "feature", "crate::Text"),
    ("GetFeaturesForBoardReplyFeatures", "featureCategory", "crate::Text"),
    ("GetFeaturesForBoardReplyFeatures", "isToggleLocked", "crate::Boolean"),
    ("GetFeaturesForBoardReplyFeatures", "prerequisites", "crate::Any"),
    ("ImportScheduleResponse", "globalId", "crate::Text"),
    ("ImportScheduleResponse", "workspaceId", "crate::Text"),
    ("ImportSourceResponse", "globalId", "crate::Text"),
    ("ImportSourceResponse", "workspaceId", "crate::Text"),
    ("IssueTransition", "isLooped", "crate::Boolean"),
    ("IssueTypeDetails", "untranslatedName", "crate::Text"),
    ("IssueTypeInfo", "entityId", "crate::Text"),
    ("IssueTypeIssueCreateMetadata", "untranslatedName", "crate::Text"),
    ("IssueTypeScheme", "issueTypes", "crate::Any"),
    ("IssueTypeScheme", "projects", "crate::Any"),
    ("IssueTypeScreenScheme", "projects", "crate::Any"),
    ("Object", "extendedInfo", "crate::Any"),
    ("Object", "name", "crate::Text"),
    ("ObjectAttribute", "objectId", "crate::Text"),
    ("ObjectAttributeValue", "referencedType", "crate::Boolean"),
    ("ObjectHistory", "affectedAttributeDisplayName", "crate::Text"),
    ("ObjectHistory", "displayValues", "crate::Any"),
    ("ObjectListInclTypeAttributesEntryResult", "hasMoreResults", "crate::Boolean"),
    ("ObjectListResult", "hasMoreResults", "crate::Boolean"),
    ("ObjectListResult", "pageSize", "crate::Integer"),
    ("ObjectSchema", "asyncTaskId", "crate::Text"),
    ("ObjectSchema", "atlassianTemplateId", "crate::Text"),
    ("ObjectSchema", "cdmData", "crate::Any"),
    ("ObjectSchema", "crossSchemaReferencingAllowed", "crate::Boolean"),
    ("ObjectType", "deletionSweepTaskId", "crate::Text"),
    ("ObjectType", "schemaDisplayLabel", "crate::Text"),
    ("ObjectType", "schemaLabel", "crate::Text"),
    ("ObjectTypeAttribute", "lockedConfiguration", "crate::Boolean"),
    ("ObjectTypeAttribute", "pendingCreation", "crate::Boolean"),
    ("PageWithCursorGetPlanResponseForPage", "isLast", "crate::Boolean"),
    ("PageWithCursorGetPlanResponseForPage", "maxResults", "crate::Integer"),
    ("PageWithCursorGetTeamResponseForPage", "isLast", "crate::Boolean"),
    ("PageWithCursorGetTeamResponseForPage", "maxResults", "crate::Integer"),
    ("PermissionHolder", "user", "crate::Any"),
    ("PrioritySchemeWithPaginatedPrioritiesAndProjects", "prioritiesLimit", "crate::Integer"),
    ("Project", "entityId", "crate::Text"),
    ("Project", "isLiveTemplate", "crate::Boolean"),
    ("Project", "prioritiesLimit", "crate::Integer"),
    ("ProjectIssueTypeHierarchy", "projectEntityId", "crate::Text"),
    ("ProjectIssueTypeHierarchy", "versionToken", "crate::Any"),
    ("PropertyKeys", "entityPropertyKeyBeans", "crate::Any"),
    ("RequestTypeDTO", "defaultName", "crate::Text"),
    ("ResolutionJSONBean", "isDefault", "crate::Boolean"),
    ("RoleActor", "user", "crate::Text"),
    ("SelfLinkDTO", "issues", "crate::Text"),
    ("ServerInformation", "defaultLocale", "crate::Any"),
    ("ServerInformation", "displayUrlCSMHelpSeeker", "crate::Text"),
    ("SourceDTO", "pageId", "crate::Text"),
    ("SourceDTO", "spaceKey", "crate::Text"),
    ("Status", "globalId", "crate::Text"),
    ("Status", "workspaceId", "crate::Text"),
    ("StatusDetails", "untranslatedName", "crate::Text"),
    ("Tickets", "allTicketsQueryBySite", "crate::Any"),
    ("ToggleFeaturesReplyFeatures", "feature", "crate::Text"),
    ("ToggleFeaturesReplyFeatures", "featureCategory", "crate::Text"),
    ("ToggleFeaturesReplyFeatures", "isToggleLocked", "crate::Boolean"),
    ("ToggleFeaturesReplyFeatures", "prerequisites", "crate::Any"),
    ("UpdatePrioritySchemeResponseBean", "updated", "crate::Any"),
    ("Version", "releaseDateSet", "crate::Boolean"),
    ("Version", "startDateSet", "crate::Boolean"),
];

pub(crate) const FIELD_KINDS: [(&str, &str, &str); 11] = [
    ("BulkOperationProgress", "created", "crate::Integer"),
    ("BulkOperationProgress", "started", "crate::Integer"),
    ("BulkOperationProgress", "updated", "crate::Integer"),
    ("Changelog", "created", "crate::Any"),
    ("ExportArchivedIssuesTaskProgressResponse", "submittedTime", "crate::Integer"),
    ("ObjectType", "parentObjectTypeId", "crate::Text"),
    ("Progress", "finishedDate", "crate::Integer"),
    ("Progress", "startDate", "crate::Integer"),
    ("TaskProgressBeanJSONNode", "result", "crate::Any"),
    ("Webhook", "expirationDate", "crate::Any"),
    ("WebhooksExpirationDate", "expirationDate", "crate::Any"),
];

pub(crate) const FIELD_WIRES: [(&str, &str, &str); 2] = [
    ("DashboardGadgetPosition", "The column position of the gadget.", "column"),
    ("DashboardGadgetPosition", "The row position of the gadget.", "row"),
];

pub(crate) const FREE_TERMS: [&str; 8] = [
    "deleteBuildsByProperty",
    "deleteByProperties",
    "deleteComponentsByProperty",
    "deleteDeploymentsByProperty",
    "deleteEntityByProperty",
    "deleteFeatureFlagsByProperty",
    "deleteRemoteLinksByProperty",
    "deleteVulnerabilitiesByProperty",
];

pub(crate) const INLINE_NAMES: [(&str, &str); 1] =
    [("StoreDevelopmentInformationReplyFailedDevinfoEntities", "FailedDevelopmentEntities")];

pub(crate) const JSON_BODIES: [&str; 2] = ["setIssueNavigatorDefaultColumns", "setUserColumns"];

pub(crate) const PAGING_ITEMS: [(&str, &str); 10] = [
    ("getAllSprints", "/values"),
    ("getAllVersions", "/values"),
    ("getBoardIssuesForEpic", "/issues"),
    ("getBoardIssuesForSprint", "/issues"),
    ("getEpics", "/values"),
    ("getIssuesForEpic", "/issues"),
    ("getIssuesForSprint", "/issues"),
    ("getIssuesWithoutEpic", "/issues"),
    ("getIssuesWithoutEpicForBoard", "/issues"),
    ("getProjects", "/values"),
];

pub(crate) const READS_BY_POST: [&str; 18] = [
    "analyseExpression",
    "bulkFetchIssues",
    "countIssues",
    "evaluateJSISJiraExpression",
    "evaluateJiraExpression",
    "getAutoCompletePost",
    "getBulkPermissions",
    "getCommentsByIds",
    "getPermittedProjects",
    "getPrecomputationsByID",
    "getWorklogsForIds",
    "matchIssues",
    "parseJqlQueries",
    "searchAndReconsileIssuesUsingJqlPost",
    "searchForIssuesUsingJqlPost",
    "searchManualRulesWithPost",
    "searchRuleSummaries",
    "searchTemplatesWithPost",
];

pub(crate) const SCHEMA_NAMES: [(&str, &str); 1] = [("fields", "MandatoryFields")];

pub(crate) const TYPE_NAMES: [(&str, &str, &str); 8] = [
    (
        "post",
        "/rest/servicedeskapi/customer/skip-permission-check",
        "CreateCustomerSkipPermissionCheck",
    ),
    (
        "post",
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/customer/skip-permission-check",
        "AddCustomersSkipPermissionCheck",
    ),
    (
        "get",
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/knowledgebase/article",
        "GetServiceDeskArticles",
    ),
    (
        "get",
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/organization",
        "GetServiceDeskOrganizations",
    ),
    (
        "get",
        "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/property",
        "GetRequestTypePropertiesKeys",
    ),
    (
        "get",
        concat!(
            "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}",
            "/property/{propertyKey}",
        ),
        "GetRequestTypeProperty",
    ),
    (
        "put",
        concat!(
            "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}",
            "/property/{propertyKey}",
        ),
        "SetRequestTypeProperty",
    ),
    (
        "delete",
        concat!(
            "/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}",
            "/property/{propertyKey}",
        ),
        "DeleteRequestTypeProperty",
    ),
];

pub(crate) const WINDOW_FIRST: [&str; 2] = ["findUserKeysByQuery", "findUsersByQuery"];

pub(crate) fn check(
    known: &BTreeSet<String>,
    fields: &BTreeSet<(String, String)>,
) -> Result<(), String> {
    assert!(!known.is_empty(), "the specifications name operations");

    let paging = PAGING_ITEMS.iter().map(|(identifier, _items)| *identifier);
    let bare = ANY_REPLIES.iter().chain(&BARE_WALKS).chain(&BINARY_REPLIES);
    let bodies = bare.chain(&BODIES_UNDECLARED).chain(&FREE_TERMS).chain(&JSON_BODIES);
    let tables = bodies.chain(&READS_BY_POST).chain(&WINDOW_FIRST).copied();
    let named = tables.chain(paging);
    let unknown: Vec<&str> = named.filter(|identifier| !known.contains(*identifier)).collect();

    if !unknown.is_empty() {
        return Err(format!("the overrides name no such operation: {}", unknown.join(", ")));
    }

    let kinds = FIELD_KINDS.iter().map(|(schema, wire, _kind)| format!("{schema}.{wire}"));
    let wires = FIELD_WIRES.iter().map(|(schema, _wire, fixed)| format!("{schema}.{fixed}"));
    let added = FIELDS_UNDECLARED.iter().map(|(schema, wire, _kind)| format!("{schema}.{wire}"));

    let held: BTreeSet<String> =
        fields.iter().map(|(schema, wire)| format!("{schema}.{wire}")).collect();

    let named_fields = kinds.chain(wires).chain(added);
    let missing: Vec<String> = named_fields.filter(|field| !held.contains(field)).collect();

    if !missing.is_empty() {
        return Err(format!("the overrides name no such field: {}", missing.join(", ")));
    }

    assert!(unknown.is_empty(), "every override names a real operation");
    assert!(missing.is_empty(), "every override names a real field");

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeSet;

    use super::{
        ANY_REPLIES,
        BARE_WALKS,
        BINARY_REPLIES,
        BODIES_UNDECLARED,
        FREE_TERMS,
        JSON_BODIES,
        PAGING_ITEMS,
        READS_BY_POST,
        WINDOW_FIRST,
        check,
    };

    #[test]
    fn an_override_that_names_no_field_is_refused() {
        let refused = check(&operations_all(), &BTreeSet::new()).expect_err("no field is known");

        assert!(refused.contains("Changelog.created"), "the failure names the kind override");
        assert!(refused.contains("DashboardGadgetPosition.row"), "the wire table is read");
        assert!(refused.contains("SourceDTO.pageId"), "the undeclared table is read");
    }

    #[test]
    fn an_override_that_names_no_operation_is_refused() {
        let known = BTreeSet::from(["getIssue".to_owned()]);

        let refused =
            check(&known, &BTreeSet::new()).expect_err("the tables name other operations");

        assert!(refused.contains("getAttachmentContent"), "the failure names the override");
        assert!(refused.contains("requesttype"), "the undeclared body table is read");
        assert!(refused.contains("setUserColumns"), "the body table is read");
        assert!(refused.contains("getDraftWorkflow"), "the reply table is read");
        assert!(refused.contains("findUsersByQuery"), "the window table is read");
        assert!(refused.contains("deleteBuildsByProperty"), "the free term table is read");
        assert!(refused.contains("searchForIssuesUsingJqlPost"), "every table is read");
    }

    fn operations_all() -> BTreeSet<String> {
        let paging = PAGING_ITEMS.iter().map(|(identifier, _items)| *identifier);

        let bare = ANY_REPLIES.iter().chain(&BARE_WALKS).chain(&BINARY_REPLIES);
        let bodies = bare.chain(&BODIES_UNDECLARED).chain(&FREE_TERMS).chain(&JSON_BODIES);
        let tables = bodies.chain(&READS_BY_POST).chain(&WINDOW_FIRST).copied();

        tables.chain(paging).map(ToOwned::to_owned).collect()
    }
}
