//! Fail-closed configuration, namespace, and deferred OSTc source-access policy.

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub enum SecurityError {
    InvalidGoal,
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
    pub safety: SafetyConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityConfig {
    pub agent_id: String,
    pub registry_number: String,
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SafetyConfig {
    pub identity_lock_required: bool,
    pub max_goal_bytes: usize,
}

pub fn validate_goal(goal: &str) -> Result<(), SecurityError> {
    let trimmed = goal.trim();
    if trimmed.is_empty()
        || trimmed.len() > crate::agent::MAX_GOAL_BYTES
        || trimmed.chars().any(char::is_control)
    {
        return Err(SecurityError::InvalidGoal);
    }
    Ok(())
}

pub fn load_config(contents: &str) -> Result<AgentConfig, SecurityError> {
    let config: AgentConfig =
        toml::from_str(contents).map_err(|_| SecurityError::InvalidConfiguration)?;
    config.validate()?;
    Ok(config)
}

impl AgentConfig {
    /// A source-access request is *always* rejected until the actual OSTc
    /// specification has been delivered, reviewed and implemented.
    pub fn require_source_access(&self) -> Result<(), SecurityError> {
        Err(SecurityError::SourceEncryptionNotImplemented)
    }

    pub fn validate(&self) -> Result<(), SecurityError> {
        if self.identity.agent_id != crate::agent::AGENT_ID
            || self.identity.registry_number != "001"
            || self.servers.agent_server != "OMAs"
            || self.servers.tool_server != "OMAt"
            || self.communication.transport != "Zenoh"
            || self.communication.network_enabled
            || !self.communication.authenticated_tls_required
            || self.source_access.enabled
            || self.source_access.encryption != "OSTc"
            || self.source_access.specification_status != "PENDING"
            || !self.safety.identity_lock_required
            || self.safety.max_goal_bytes != crate::agent::MAX_GOAL_BYTES
        {
            return Err(SecurityError::InvalidConfiguration);
        }
        Ok(())
    }
}

pub fn safe_namespace_part(value: &str) -> Result<(), SecurityError> {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(SecurityError::InvalidRoute);
    }
    Ok(())
}

pub fn safe_tool_name(value: &str) -> Result<(), SecurityError> {
    if value.is_empty()
        || value.len() > 128
        || value
            .split('.')
            .any(|part| safe_namespace_part(part).is_err())
    {
        return Err(SecurityError::InvalidRoute);
    }
    Ok(())
}
