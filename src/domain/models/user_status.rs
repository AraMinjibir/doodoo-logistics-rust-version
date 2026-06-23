use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    CustomerSupportAgent,
    Recipient,
    Sender,
    ServiceProvider,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Suspended,
    Deleted,
}

impl UserRole {
    pub fn string_roles(roles: &str) -> Option<Self> {
        match roles {
            "Admin" => Some(Self::Admin),
            "CustomerSupportAgent" => Some(Self::CustomerSupportAgent),
            "Recipient" => Some(Self::Recipient),
            "Sender" => Some(Self::Sender),
            "ServiceProvider" => Some(Self::ServiceProvider),
            _ => None,
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Admin => "Admin",
            Self::CustomerSupportAgent => "CustomerSupportAgent",
            Self::Sender => "Sender",
            Self::ServiceProvider => "ServiceProvider",
            Self::Recipient => "Recipient",
        };
        write!(f, "{}", value)
    }
}
impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::string_roles(s).ok_or_else(|| format!("Invalid role: {}", s))
    }
}

impl UserStatus {
    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "Active" => Some(Self::Active),
            "Suspended" => Some(Self::Suspended),
            "Deleted" => Some(Self::Deleted),
            _ => None,
        }
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Active => "Active",
            Self::Suspended => "Suspended",
            Self::Deleted => "Deleted",
        };
        write!(f, "{}", value)
    }
}
impl FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s).ok_or_else(|| format!("Invalid status: {}", s))
    }
}
