use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use crate::domain::errors::domain_error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShipmentStatus {
    Created,
    Assigned,
    InTransit,
    OutForDelivery,
    Delivered,
    Cancelled,
}

impl ShipmentStatus {
    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "Created" => Some(Self::Created),
            "Assigned" => Some(Self::Assigned),
            "InTransit" => Some(Self::InTransit),
            "OutForDelivery" => Some(Self::OutForDelivery),
            "Delivered" => Some(Self::Delivered),
            "Cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }

    pub fn validate_transition(
        current: &ShipmentStatus,
        next: &ShipmentStatus,
    ) -> Result<(), DomainError> {
        let allowed = ALLOWED_TRANSITIONS.get(current).unwrap_or(&EMPTY_SET);

        if !allowed.contains(next) {
            return Err(DomainError::InvalidShipmentStatusTransition {
                from: current.clone(),
                to: next.clone(),
            });
        }

        Ok(())
    }
}

impl fmt::Display for ShipmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Created => "Created",
            Self::Assigned => "Assigned",
            Self::InTransit => "InTransit",
            Self::OutForDelivery => "OutForDelivery",
            Self::Delivered => "Delivered",
            Self::Cancelled => "Cancelled",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for ShipmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s).ok_or_else(|| format!("Invalid status: {}", s))
    }
}

static ALLOWED_TRANSITIONS: Lazy<HashMap<ShipmentStatus, HashSet<ShipmentStatus>>> =
    Lazy::new(|| {
        use ShipmentStatus::*;

        HashMap::from([
            (Created, HashSet::from([Assigned, InTransit, Cancelled])),
            (
                Assigned,
                HashSet::from([InTransit, OutForDelivery, Cancelled]),
            ),
            (
                InTransit,
                HashSet::from([OutForDelivery, Delivered, Cancelled]),
            ),
            (OutForDelivery, HashSet::from([Delivered, Cancelled])),
            (Delivered, HashSet::new()),
            (Cancelled, HashSet::new()),
        ])
    });

static EMPTY_SET: Lazy<HashSet<ShipmentStatus>> = Lazy::new(HashSet::new);
