use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;

use std::num::ParseIntError;
use std::{collections::HashSet, net::IpAddr, str::FromStr};

use crate::permissions;
use crate::qradar::qradar_mock::QRadarMock;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ReferenceSetError {
    #[error("type mismatch: {0}")]
    TypeMismatch(String),

    #[error("reference set {0:?} doesn't exists")]
    ReferenceSetDoesNotExists(String),

    #[error("reference set {0:?} already exists")]
    ReferenceSetAlreadyExists(String),

    #[error("entry {0:?} was not found")]
    EntryNotFound(String),

    #[error("provided unsupported type {0:?}")]
    UnsupportedType(String),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) enum ReferenceSet {
    AlphaNumeric(HashSet<String>),
    AlphaNumericIgnoreCase(HashSet<String>),
    Numeric(HashSet<i64>),
    Port(HashSet<u16>),
    Ip(HashSet<IpAddr>),
}

impl FromStr for ReferenceSet {
    type Err = ReferenceSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALN" => Ok(ReferenceSet::AlphaNumeric(HashSet::new())),
            "NUM" => Ok(ReferenceSet::Numeric(HashSet::new())),
            "IP" => Ok(ReferenceSet::Ip(HashSet::new())),
            "PORT" => Ok(ReferenceSet::Port(HashSet::new())),
            "ALNIC" => Ok(ReferenceSet::AlphaNumericIgnoreCase(HashSet::new())),
            unknown_type => Err(ReferenceSetError::UnsupportedType(unknown_type.into())),
        }
    }
}

impl QRadarMock {
    pub(crate) fn add_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: String,
        reference_set: ReferenceSet,
    ) -> Result<(), ReferenceSetError> {
        let entry = self
            .reference_sets_write_access(authorization_token)
            .entry(name);

        match entry {
            Occupied(value) => Err(ReferenceSetError::ReferenceSetAlreadyExists(
                value.key().to_owned(),
            )),
            Vacant(value) => {
                value.insert(reference_set);
                Ok(())
            }
        }
    }

    pub(crate) fn get_reference_set(
        &self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
    ) -> Option<&ReferenceSet> {
        self.reference_sets_readonly_access(authorization_token)
            .get(name)
    }

    pub(crate) fn delete_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
    ) -> Result<(), ReferenceSetError> {
        self.reference_sets_write_access(authorization_token)
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| ReferenceSetError::ReferenceSetDoesNotExists(name.to_string()))
    }

    pub(crate) fn insert_to_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
        value: &str,
    ) -> Result<bool, ReferenceSetError> {
        let maybe_reference_set_write_access = self
            .reference_sets_write_access(authorization_token)
            .get_mut(name);

        match maybe_reference_set_write_access {
            Some(reference_set_type) => match reference_set_type {
                ReferenceSet::AlphaNumeric(reference_set) => {
                    Ok(reference_set.insert(value.to_string()))
                }

                ReferenceSet::AlphaNumericIgnoreCase(reference_set) => {
                    Ok(reference_set.insert(value.to_lowercase().to_string()))
                }

                ReferenceSet::Numeric(reference_set) => {
                    Ok(reference_set.insert(value.parse().map_err(|_| {
                        ReferenceSetError::TypeMismatch(format!(
                            "{name}: {value:?} is not a number"
                        ))
                    })?))
                }

                ReferenceSet::Port(reference_set) => {
                    Ok(reference_set.insert(value.parse().map_err(|_| {
                        ReferenceSetError::TypeMismatch(format!(
                            "{name}: {value:?} is not a port number"
                        ))
                    })?))
                }

                ReferenceSet::Ip(reference_set) => {
                    Ok(reference_set.insert(value.parse().map_err(|_| {
                        ReferenceSetError::TypeMismatch(format!(
                            "{name}: {value:?} is not an IP address"
                        ))
                    })?))
                }
            },
            None => Err(ReferenceSetError::ReferenceSetDoesNotExists(
                name.to_string(),
            )),
        }
    }

    pub(crate) fn delete_from_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
        value: &str,
    ) -> Result<(), ReferenceSetError> {
        let maybe_reference_set_write_access = self
            .reference_sets_write_access(authorization_token)
            .get_mut(name);

        match maybe_reference_set_write_access {
            Some(reference_set_type) => match reference_set_type {
                ReferenceSet::AlphaNumeric(reference_set) => reference_set
                    .remove(value)
                    .then_some(())
                    .ok_or_else(|| ReferenceSetError::EntryNotFound(value.to_string())),

                ReferenceSet::AlphaNumericIgnoreCase(reference_set) => reference_set
                    .remove(value.to_lowercase().as_str())
                    .then_some(())
                    .ok_or_else(|| ReferenceSetError::EntryNotFound(value.to_string())),

                ReferenceSet::Numeric(reference_set) => reference_set
                    .remove(&value.parse().map_err(|_| {
                        ReferenceSetError::TypeMismatch(format!(
                            "{name}: {value:?} is not a number"
                        ))
                    })?)
                    .then_some(())
                    .ok_or_else(|| ReferenceSetError::EntryNotFound(value.to_string())),

                ReferenceSet::Port(reference_set) => reference_set
                    .remove(&value.parse().map_err(|_| {
                        ReferenceSetError::TypeMismatch(format!(
                            "{name}: {value:?} is not a port number"
                        ))
                    })?)
                    .then_some(())
                    .ok_or_else(|| ReferenceSetError::EntryNotFound(value.to_string())),

                ReferenceSet::Ip(reference_set) => reference_set
                    .remove(&value.parse().map_err(|_| {
                        ReferenceSetError::TypeMismatch(format!(
                            "{name}: {value:?} is not an IP address"
                        ))
                    })?)
                    .then_some(())
                    .ok_or_else(|| ReferenceSetError::EntryNotFound(value.to_string())),
            },
            None => Err(ReferenceSetError::ReferenceSetDoesNotExists(
                name.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        permissions::{Authentication, AuthorizationToken, REGISTERED_TOKEN},
        qradar::{
            qradar_mock::QRadarMock,
            reference_data::sets::{ReferenceSet, ReferenceSetError},
        },
    };

    const TEST_REFERENCE_SET_NAME: &str = "test_set";

    #[test]
    fn add_reference_set_success() {
        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(add_result.is_ok());
    }

    #[test]
    fn add_reference_set_double_insert_failure() {
        let test_reference_set_name = TEST_REFERENCE_SET_NAME.to_string();

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            test_reference_set_name.clone(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let add_result = mock.add_reference_set(
            authorization_token,
            test_reference_set_name.clone(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(matches!(
            add_result,
            Err(ReferenceSetError::ReferenceSetAlreadyExists(reference_set_name)) if reference_set_name == test_reference_set_name
        ));
    }

    #[test]
    fn get_reference_set_success() {
        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        // Downgrade privilege to readonly
        let mock = mock;

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let maybe_reference_set =
            mock.get_reference_set(authorization_token, TEST_REFERENCE_SET_NAME);

        assert!(maybe_reference_set.is_some())
    }

    #[test]
    fn delete_reference_set_success() {
        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let delete_result = mock.delete_reference_set(authorization_token, TEST_REFERENCE_SET_NAME);

        assert!(delete_result.is_ok());
    }

    #[test]
    fn delete_reference_set_does_not_exist_failure() {
        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let delete_result = mock.delete_reference_set(authorization_token, TEST_REFERENCE_SET_NAME);

        assert!(
            matches!(delete_result, Err(ReferenceSetError::ReferenceSetDoesNotExists(reference_set_name)) if reference_set_name == TEST_REFERENCE_SET_NAME)
        );
    }

    #[test]
    fn insert_to_reference_set_success() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let result =
            mock.insert_to_reference_set(authorization_token, TEST_REFERENCE_SET_NAME, test_value);

        assert!(result.is_ok());
    }

    #[test]
    fn insert_to_reference_set_missing_set_failure() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let result =
            mock.insert_to_reference_set(authorization_token, TEST_REFERENCE_SET_NAME, test_value);

        assert!(
            matches!(result, Err(ReferenceSetError::ReferenceSetDoesNotExists(reference_set_name)) if reference_set_name == TEST_REFERENCE_SET_NAME)
        );
    }

    #[test]
    fn insert_to_reference_set_wrong_type_failure() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::Numeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let result =
            mock.insert_to_reference_set(authorization_token, TEST_REFERENCE_SET_NAME, test_value);

        assert!(
            matches!(result, Err(ReferenceSetError::TypeMismatch(error_message)) if error_message == format!("{TEST_REFERENCE_SET_NAME}: {test_value:?} is not a number"))
        );
    }

    #[test]
    fn delete_from_reference_set_success() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let result =
            mock.insert_to_reference_set(authorization_token, TEST_REFERENCE_SET_NAME, test_value);

        assert!(result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let delete_result = mock.delete_from_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME,
            test_value,
        );

        assert!(delete_result.is_ok());
    }

    #[test]
    fn delete_from_reference_set_entry_does_not_exist_failure() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let delete_result = mock.delete_from_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME,
            test_value,
        );

        assert!(
            matches!(delete_result, Err(ReferenceSetError::EntryNotFound(entry)) if entry == test_value)
        );
    }

    #[test]
    fn delete_from_reference_set_that_does_not_exist_failure() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let delete_result = mock.delete_from_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME,
            test_value,
        );

        assert!(
            matches!(delete_result, Err(ReferenceSetError::ReferenceSetDoesNotExists(reference_set)) if reference_set == TEST_REFERENCE_SET_NAME)
        );
    }

    #[test]
    fn delete_from_reference_set_wrong_type_failure() {
        let test_value = "test value";

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let mut mock = QRadarMock::new();

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::Numeric(HashSet::new()),
        );

        assert!(add_result.is_ok());

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let delete_result = mock.delete_from_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME,
            test_value,
        );

        assert!(
            matches!(delete_result, Err(ReferenceSetError::TypeMismatch(error_message)) if error_message == format!("{TEST_REFERENCE_SET_NAME}: {test_value:?} is not a number"))
        )
    }
}
