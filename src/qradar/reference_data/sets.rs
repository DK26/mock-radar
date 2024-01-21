use std::{collections::HashSet, net::IpAddr, str::FromStr};

use crate::permissions;
use crate::qradar::qradar_mock::QRadarMock;

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) enum ReferenceSet {
    AlphaNumeric(HashSet<String>),
    AlphaNumericIgnoreCase(HashSet<String>),
    Numeric(HashSet<i64>),
    Port(HashSet<u16>),
    Ip(HashSet<IpAddr>),
}

impl FromStr for ReferenceSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALN" => Ok(ReferenceSet::AlphaNumeric(HashSet::new())),
            "NUM" => Ok(ReferenceSet::Numeric(HashSet::new())),
            "IP" => Ok(ReferenceSet::Ip(HashSet::new())),
            "PORT" => Ok(ReferenceSet::Port(HashSet::new())),
            "ALNIC" => Ok(ReferenceSet::AlphaNumericIgnoreCase(HashSet::new())),
            _ => Err(anyhow::anyhow!("unable to parse reference set type")),
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
        name: String,
        value: &str,
    ) -> anyhow::Result<()> {
        todo!("attempt to parse the provided string value into the the retrieved ReferenceSet type and append")
    }

    pub(crate) fn remove_from_reference_set(
        &mut self,
        authorization_token: permissions::AuthorizationToken,
        name: &str,
    ) -> anyhow::Result<()> {
        todo!()
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
