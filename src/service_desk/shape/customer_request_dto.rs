#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestDTO;

impl CustomerRequestDTO {
    pub const ACTIONS: crate::Field<Self, crate::Object<super::CustomerRequestActionsDTO>> =
        crate::Field::new("actions");

    pub const ATTACHMENTS: crate::Field<Self, crate::Object<super::PagedDTOAttachmentDTO>> =
        crate::Field::new("attachments");

    pub const COMMENTS: crate::Field<Self, crate::Object<super::PagedDTOCommentDTO>> =
        crate::Field::new("comments");

    pub const CREATED_DATE: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("createdDate");

    pub const EXPANDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("_expands");
    pub const ISSUE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueId");
    pub const ISSUE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("issueKey");

    pub const LINKS: crate::Field<Self, crate::Object<super::CustomerRequestLinkDTO>> =
        crate::Field::new("_links");

    pub const PARTICIPANTS: crate::Field<Self, crate::Object<super::PagedDTOUserDTO>> =
        crate::Field::new("participants");

    pub const REPORTER: crate::Field<Self, crate::Object<super::UserDTO>> =
        crate::Field::new("reporter");

    pub const REQUEST_FIELD_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::CustomerRequestFieldValueDTO>>,
    > = crate::Field::new("requestFieldValues");

    pub const REQUEST_TYPE: crate::Field<Self, crate::Object<super::RequestTypeDTO>> =
        crate::Field::new("requestType");

    pub const REQUEST_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("requestTypeId");

    pub const SERVICE_DESK: crate::Field<Self, crate::Object<super::ServiceDeskDTO>> =
        crate::Field::new("serviceDesk");

    pub const SERVICE_DESK_ID: crate::Field<Self, crate::Text> = crate::Field::new("serviceDeskId");

    pub const SLA: crate::Field<Self, crate::Object<super::PagedDTOSlaInformationDTO>> =
        crate::Field::new("sla");

    pub const STATUS: crate::Field<Self, crate::Object<super::PagedDTOCustomerRequestStatusDTO>> =
        crate::Field::new("status");

    pub const STATUS_CURRENT: crate::Field<Self, crate::Object<super::CustomerRequestStatusDTO>> =
        crate::Field::new("currentStatus");

    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
}
