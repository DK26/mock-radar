pub(crate) mod post_reference_set_with_readonly_sec_token_forbidden_failure;
pub(crate) mod post_reference_set_with_sec_token_conflict_failure;
pub(crate) mod post_reference_set_with_sec_token_invalid_element_type_failure;
pub(crate) mod post_reference_set_with_sec_token_missing_element_type_param_failure;
pub(crate) mod post_reference_set_with_sec_token_missing_name_param_failure;
pub(crate) mod post_reference_set_with_sec_token_success;
pub(crate) mod post_reference_set_with_sec_token_without_params_failure;

use serde::Deserialize;

// TODO: Move types to a module of their own
/// This type exists only as workaround to ignore the dynamic value of `creation_time` while validating it.
/// If a better or more proper approach will be found in the future, this may get removed.
#[derive(Debug, Deserialize)]
pub(crate) struct TestPostResponse {
    timeout_type: String,
    number_of_elements: u32,
    #[allow(unused)]
    creation_time: u128, // Assuming successful deserialization implies a valid timestamp
    name: String,
    element_type: String,
}

impl PartialEq for TestPostResponse {
    fn eq(&self, other: &Self) -> bool {
        self.timeout_type == other.timeout_type
            && self.number_of_elements == other.number_of_elements
            && self.name == other.name
            && self.element_type == other.element_type
        // Ignoring `creation_time` in comparison
    }
}
