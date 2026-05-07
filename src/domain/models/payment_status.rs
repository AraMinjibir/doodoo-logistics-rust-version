use std::{
    fmt,
    str::FromStr,
    collections::HashMap,
    collections::HashSet,
};
use once_cell::sync::Lazy;


use crate::domain::errors::domain_error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentStatus {
    Pending,
    Successful,
    Failed, 
    Refunded
     }

 impl PaymentStatus {
     pub fn statuses() -> &'static [PaymentStatus] {

        &[
            Self::Pending,
            Self::Successful,
            Self::Failed,
            Self::Refunded
        ]
     }

     pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "Pending" => Some(Self::Pending),
            "Successful" => Some(Self::Successful),
            "Failed" => Some(Self::Failed),
            "Refunded" => Some(Self::Refunded),
            _ => None
            
        }
     }

     pub fn validate_transition(
        current: &PaymentStatus,
        next: &PaymentStatus,
        ) -> Result<(), DomainError> {
        let allowed = ALLOWED_TRANSITIONS
            .get(current)
            .unwrap_or(&EMPTY_SET);
    
        if !allowed.contains(next) {
            return Err(DomainError::InvalidPaymentStatusTransition {
                from: current.clone(),
                to: next.clone(),
            });
        }
    
        Ok(())
    }


 }    

 impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Pending => "Pending",
            Self::Successful => "Successful",
            Self::Failed => "Failed",
            Self::Refunded => "Refunded"
        };
        write!(f, "{}", value)
    }
}

impl FromStr for PaymentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
            .ok_or_else(|| format!("Invalid status: {}", s))
    }
}

 static ALLOWED_TRANSITIONS: Lazy<HashMap<PaymentStatus, HashSet<PaymentStatus>>> =
    Lazy::new(|| {
        use PaymentStatus::*;

        HashMap::from([
            (Pending, HashSet::from([Failed, Refunded, Successful])),
            (Failed, HashSet::from([Refunded])),
            (Refunded, HashSet::from([Pending, Successful])),
            (Successful, HashSet::new()),
        ])
    });

    static EMPTY_SET: Lazy<HashSet<PaymentStatus>> = Lazy::new(HashSet::new);