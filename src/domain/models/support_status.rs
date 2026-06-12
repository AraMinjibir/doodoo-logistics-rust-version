use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use crate::domain::errors::domain_error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SupportStatus {
    Open,
    InProgress,
    Resolved,
    Cancelled,
}

impl SupportStatus {
    pub fn values() -> &'static [SupportStatus] {
        &[
            Self::Open,
            Self::InProgress,
            Self::Resolved,
            Self::Cancelled,
        ]
    }

    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "Open" => Some(Self::Open),
            "InProgress" => Some(Self::InProgress),
            "Resolved" => Some(Self::Resolved),
            "Cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

pub fn validate_transition(
    current: &SupportStatus,
    next: &SupportStatus,
) -> Result<(), DomainError> {
    let allowed = ALLOWED_TRANSITIONS.get(current).unwrap_or(&EMPTY_SET);

    if !allowed.contains(next) {
        return Err(DomainError::InvalidSupportStatusTransition {
            from: current.clone(),
            to: next.clone(),
        });
    }

    Ok(())
}

impl fmt::Display for SupportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Open => "Open",
            Self::InProgress => "InProgress",
            Self::Resolved => "Resolved",
            Self::Cancelled => "Cancelled",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for SupportStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s).ok_or_else(|| format!("Invalid status: {}", s))
    }
}

static ALLOWED_TRANSITIONS: Lazy<HashMap<SupportStatus, HashSet<SupportStatus>>> =
    Lazy::new(|| {
        use SupportStatus::*;

        HashMap::from([
            (Open, HashSet::from([InProgress, Resolved, Cancelled])),
            (InProgress, HashSet::from([Resolved, Cancelled])),
        ])
    });

static EMPTY_SET: Lazy<HashSet<SupportStatus>> = Lazy::new(HashSet::new);
