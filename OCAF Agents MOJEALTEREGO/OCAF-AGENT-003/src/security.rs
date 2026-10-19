//! Configuration authorization is fail-closed. No OSTc specification has been supplied.
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    InvalidConfiguration,
    SourceEncryptionNotImplemented,
    InvalidRoute,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentConfig {
    pub identity: IdentityConfig,
    pub servers: ServersConfig,
    pub communication: CommunicationConfig,
    pub source_access: SourceAccessConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityConfig {
    pub registry_number: String,
    pub agent_id: String,
    pub unique_name: String,
    pub serial_number: String,
    pub catalog_number: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServersConfig {
    pub agent_server: String,
    pub tool_server: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommunicationConfig {
    pub transport: String,
    pub network_enabled: bool,
    pub authenticated_tls_required: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAccessConfig {
    pub enabled: bool,
    pub encryption: String,
    pub specification_status: String,
}

pub fn load_config(raw: &str) -> Result<AgentConfig, SecurityError> {
    let conf: AgentConfig = toml::from_str(raw).map_err(|_| SecurityError::InvalidConfiguration)?;
    if conf.identity.registry_number != crate::agent::REGISTRY_NUMBER
        || conf.identity.agent_id != crate::agent::AGENT_ID
        || conf.identity.unique_name != crate::agent::AGENT_NAME
        || conf.identity.serial_number != crate::agent::SERIAL_NUMBER
        || conf.identity.catalog_number != crate::agent::CATALOG_NUMBER
        || conf.servers.agent_server != "OMAs"
        || conf.servers.tool_server != "OMAt"
        || conf.communication.transport != "Zenoh"
        || conf.communication.network_enabled
        || !conf.communication.authenticated_tls_required
        || conf.source_access.enabled
        || conf.source_access.encryption != "OSTc"
        || conf.source_access.specification_status != "PENDING"
    {
        return Err(SecurityError::InvalidConfiguration);
    }
    Ok(conf)
}

impl AgentConfig {
    pub fn source_access(&self) -> Result<(), SecurityError> {
        // No fallback cipher; deny until signed OSTc implementation and tests.
        Err(SecurityError::SourceEncryptionNotImplemented)
    }
}

pub fn safe_segment(value: &str) -> Result<(), SecurityError> {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err(SecurityError::InvalidRoute);
    }
    Ok(())
}

pub fn safe_tool_name(value: &str) -> Result<(), SecurityError> {
    if value.is_empty()
        || value.len() > 128
        || value.split('.').any(|part| safe_segment(part).is_err())
    {
        return Err(SecurityError::InvalidRoute);
    }
    Ok(())
}
