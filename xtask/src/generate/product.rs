pub(crate) const PRODUCTS: [Product; 5] = [
    Product {
        base: "Jira",
        is_experimental_header_sent: false,
        module: "assets",
        path_prefix: "/jsm/assets/workspace/{workspaceId}/v1",
    },
    Product {
        base: "Automation",
        is_experimental_header_sent: false,
        module: "automation",
        path_prefix: "",
    },
    Product {
        base: "Jira",
        is_experimental_header_sent: false,
        module: "platform",
        path_prefix: "",
    },
    Product {
        base: "Jira",
        is_experimental_header_sent: true,
        module: "service_desk",
        path_prefix: "",
    },
    Product {
        base: "Jira",
        is_experimental_header_sent: false,
        module: "software",
        path_prefix: "",
    },
];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Product {
    pub base: &'static str,
    pub is_experimental_header_sent: bool,
    pub module: &'static str,
    pub path_prefix: &'static str,
}
