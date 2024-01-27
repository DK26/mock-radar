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

    #[error("entry {0:?} doesn't exists")]
    EntryDoesNotExists(String),

    #[error("entry {0:?} already exists")]
    EntryAlreadyExists(String),

    #[error("provided unsupported type {0:?}")]
    UnsupportedType(String),

    #[error(transparent)]
    System(#[from] anyhow::Error),
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
        let entry = self.write_reference_sets(authorization_token).entry(name);

        match entry {
            Occupied(value) => Err(ReferenceSetError::EntryAlreadyExists(
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
        self.readonly_reference_sets(authorization_token).get(name)
    }

    pub(crate) fn delete_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
    ) -> Result<(), ReferenceSetError> {
        self.write_reference_sets(authorization_token)
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| ReferenceSetError::EntryDoesNotExists(name.to_string()))
    }

    pub(crate) fn insert_to_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
        value: &str,
    ) -> Result<bool, ReferenceSetError> {
        let maybe_write_reference_set =
            self.write_reference_sets(authorization_token).get_mut(name);

        match maybe_write_reference_set {
            Some(reference_set) => match reference_set {
                ReferenceSet::AlphaNumeric(set) => Ok(set.insert(value.to_string())),
                ReferenceSet::AlphaNumericIgnoreCase(set) => {
                    Ok(set.insert(value.to_lowercase().to_string()))
                }
                ReferenceSet::Numeric(set) => Ok(set.insert(
                    value
                        .parse()
                        .map_err(|e| ReferenceSetError::TypeMismatch(format!("{e:#?}")))?,
                )),
                ReferenceSet::Port(set) => Ok(set.insert(
                    value
                        .parse()
                        .map_err(|e| ReferenceSetError::TypeMismatch(format!("{e:#?}")))?,
                )),
                ReferenceSet::Ip(set) => Ok(set.insert(
                    value
                        .parse()
                        .map_err(|e| ReferenceSetError::TypeMismatch(format!("{e:#?}")))?,
                )),
            },
            None => Err(ReferenceSetError::EntryDoesNotExists(name.to_string())),
        }
    }

    pub(crate) fn delete_from_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
        value: &str,
    ) -> anyhow::Result<()> {
        todo!()
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
        let mut mock = QRadarMock::new();

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

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

        let mut mock = QRadarMock::new();

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

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
            Err(ReferenceSetError::EntryAlreadyExists(reference_set_name)) if reference_set_name == test_reference_set_name
        ));
    }

    #[test]
    fn get_reference_set_success() {
        let mut mock = QRadarMock::new();

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let add_result = mock.add_reference_set(
            authorization_token,
            TEST_REFERENCE_SET_NAME.to_string(),
            ReferenceSet::AlphaNumeric(HashSet::new()),
        );

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
        let mut mock = QRadarMock::new();

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

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
        let mut mock = QRadarMock::new();

        let authorization_token =
            AuthorizationToken::validate(Authentication::Token(REGISTERED_TOKEN.to_string()))
                .expect("failed authentication");

        let delete_result = mock.delete_reference_set(authorization_token, TEST_REFERENCE_SET_NAME);

        assert!(
            matches!(delete_result, Err(ReferenceSetError::EntryDoesNotExists(reference_set_name)) if reference_set_name == TEST_REFERENCE_SET_NAME)
        );
    }
}
