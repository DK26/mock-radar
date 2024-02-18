pub(crate) mod post_reference_set_with_readonly_sec_token_forbidden_failure;
pub(crate) mod post_reference_set_with_sec_token_all_options_unfiltered_success;
pub(crate) mod post_reference_set_with_sec_token_conflict_failure;
pub(crate) mod post_reference_set_with_sec_token_fields_multiple_select_timeout_type_and_name_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_creation_time_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_element_type_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_name_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_number_of_elements_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_time_to_live_empty_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_time_to_live_success;
pub(crate) mod post_reference_set_with_sec_token_fields_select_timeout_type_success;
pub(crate) mod post_reference_set_with_sec_token_fields_selection_failures;
pub(crate) mod post_reference_set_with_sec_token_request_invalidation_sequence_failure;
pub(crate) mod post_reference_set_with_sec_token_success;
pub(crate) mod post_reference_set_with_sec_token_without_params_failure;

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct CreationTime {
    #[allow(unused)]
    pub(crate) creation_time: u64, // Assuming successful deserialization implies a valid timestamp
}

impl PartialEq for CreationTime {
    fn eq(&self, _: &Self) -> bool {
        true
        // Ignoring `creation_time` in comparison
    }
}

// TODO: Move types to a module of their own
/// This type exists only as workaround to ignore the dynamic value of `creation_time` while validating it.
/// If a better or more proper approach will be found in the future, this may get removed.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct TestPostResponse {
    timeout_type: String,
    time_to_live: Option<String>,
    number_of_elements: u32,

    #[serde(flatten)]
    creation_time: CreationTime,

    name: String,
    element_type: String,
}
