use std::num::ParseIntError;
use std::{collections::HashSet, net::IpAddr, str::FromStr};

use crate::permissions;
use crate::qradar::qradar_mock::QRadarMock;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ReferenceSetError {
    #[error("type mismatch: {0}")]
    TypeMismatch(String),

    #[error("entry doesn't exists")]
    EntryDoesNotExists,

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
    ) -> Option<ReferenceSet> {
        self.write_reference_sets(authorization_token)
            .insert(name, reference_set)
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
    ) -> Option<ReferenceSet> {
        self.write_reference_sets(authorization_token).remove(name)
    }

    pub(crate) fn insert_to_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
        value: &str,
    ) -> anyhow::Result<bool, ReferenceSetError> {
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
            None => Err(ReferenceSetError::EntryDoesNotExists),
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
mod tests {}
