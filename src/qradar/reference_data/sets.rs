use std::{collections::HashSet, net::IpAddr, str::FromStr};

use crate::permissions;
use crate::qradar::QRadarMock;

#[derive(Eq, PartialEq, Debug)]
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
        name: String,
        reference_set: ReferenceSet,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) fn get_reference_set(&mut self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) fn delete_reference_set(&mut self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) fn insert_to_reference_set(
        &mut self,
        _: permissions::AuthorizationToken,
        name: String,
        value: &str,
    ) -> anyhow::Result<()> {
        todo!("attempt to parse the provided string value into the the retrieved ReferenceSet type and append")
    }

    pub(crate) fn remove_from_reference_set(&mut self, name: &str) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) fn delete_from_reference_set(
        &mut self,
        name: &str,
        value: &str,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
