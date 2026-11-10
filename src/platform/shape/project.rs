#[derive(Clone, Copy, Debug)]
pub struct Project;

impl Project {
    pub const ARCHIVED: crate::Field<Self, crate::Boolean> = crate::Field::new("archived");

    pub const ARCHIVED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("archivedBy");

    pub const ARCHIVED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("archivedDate");
    pub const ASSIGNEE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("assigneeType");

    pub const AVATAR_URLS: crate::Field<Self, crate::Object<super::AvatarURLsBean>> =
        crate::Field::new("avatarUrls");

    pub const COMPONENTS: crate::Field<Self, crate::List<crate::Object<super::ProjectComponent>>> =
        crate::Field::new("components");

    pub const DELETED: crate::Field<Self, crate::Boolean> = crate::Field::new("deleted");

    pub const DELETED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("deletedBy");

    pub const DELETED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("deletedDate");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EMAIL: crate::Field<Self, crate::Text> = crate::Field::new("email");
    pub const ENTITY_ID: crate::Field<Self, crate::Text> = crate::Field::new("entityId");
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const FAVOURITE: crate::Field<Self, crate::Boolean> = crate::Field::new("favourite");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const INSIGHT: crate::Field<Self, crate::Object<super::ProjectInsight>> =
        crate::Field::new("insight");

    pub const ISSUE_TYPES: crate::Field<Self, crate::List<crate::Object<super::IssueTypeDetails>>> =
        crate::Field::new("issueTypes");

    pub const ISSUE_TYPE_HIERARCHY: crate::Field<Self, crate::Object<super::Hierarchy>> =
        crate::Field::new("issueTypeHierarchy");

    pub const IS_LIVE_TEMPLATE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isLiveTemplate");

    pub const IS_PRIVATE: crate::Field<Self, crate::Boolean> = crate::Field::new("isPrivate");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const LANDING_PAGE_INFORMATION: crate::Field<
        Self,
        crate::Object<super::ProjectLandingPageInfo>,
    > = crate::Field::new("landingPageInfo");

    pub const LEAD: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("lead");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PERMISSIONS: crate::Field<Self, crate::Object<super::ProjectPermissions>> =
        crate::Field::new("permissions");

    pub const PRIORITIES_LIMIT: crate::Field<Self, crate::Integer> =
        crate::Field::new("prioritiesLimit");

    pub const PROJECT_CATEGORY: crate::Field<Self, crate::Object<super::ProjectCategory>> =
        crate::Field::new("projectCategory");

    pub const PROJECT_TYPE_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectTypeKey");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const RETENTION_TILL_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("retentionTillDate");

    pub const ROLES: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("roles");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const SIMPLIFIED: crate::Field<Self, crate::Boolean> = crate::Field::new("simplified");
    pub const STYLE: crate::Field<Self, crate::Text> = crate::Field::new("style");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
    pub const UUID: crate::Field<Self, crate::Text> = crate::Field::new("uuid");

    pub const VERSIONS: crate::Field<Self, crate::List<crate::Object<super::Version>>> =
        crate::Field::new("versions");
}
