const GATEWAY_URL: &str = "https://api.atlassian.com";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Base {
    Automation,
    Jira,
}

impl Base {
    #[must_use]
    pub(crate) fn url(self, cloud_id: &str) -> String {
        assert!(!cloud_id.is_empty(), "a base names its site");
        assert!(!cloud_id.contains('/'), "a site id never leaves its path segment");

        let url = match self {
            Self::Automation => format!("{GATEWAY_URL}/automation/public/jira/{cloud_id}"),
            Self::Jira => format!("{GATEWAY_URL}/ex/jira/{cloud_id}"),
        };

        assert!(url.starts_with(GATEWAY_URL), "every request stays on the scoped token gateway");
        assert!(url.ends_with(cloud_id), "a base ends at the site it names");

        url
    }
}

#[cfg(test)]
mod tests {
    use super::Base;

    #[test]
    #[should_panic(expected = "a site id never leaves its path segment")]
    fn a_site_id_carrying_a_separator_is_a_programmer_error() {
        let _url = Base::Jira.url("tenant/../other");
    }

    #[test]
    fn each_base_stays_on_the_gateway_and_names_the_site() {
        assert_eq!(Base::Jira.url("site"), "https://api.atlassian.com/ex/jira/site", "the REST");

        assert_eq!(
            Base::Automation.url("site"),
            "https://api.atlassian.com/automation/public/jira/site",
            "the automation API",
        );
    }
}
