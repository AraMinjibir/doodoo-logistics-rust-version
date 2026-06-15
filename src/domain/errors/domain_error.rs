use chrono::NaiveDate;
use std::fmt::{self};
use uuid::Uuid;

use crate::domain::errors::repository_error::RepositoryError;
use crate::domain::models::payment_status::PaymentStatus;
use crate::domain::models::shipment_status::ShipmentStatus;
use crate::domain::models::support_status::SupportStatus;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DomainError {
    ShipmentNotFound {
        tracking_number: String,
    },

    ShipmentNotFoundById {
        id: Uuid,
    },
    ShipmentNotDelivered,

    InvalidShipmentStatusTransition {
        from: ShipmentStatus,
        to: ShipmentStatus,
    },
    InvalidPaymentStatusTransition {
        from: PaymentStatus,
        to: PaymentStatus,
    },
    InvalidSupportStatusTransition {
        from: SupportStatus,
        to: SupportStatus,
    },
    PaymentExistsForThisShipment {
        id: Uuid,
    },
    PaymentNotFound {
        reference: String,
    },
    PaymentWithShipmentIdNotFound {
        shipment_id: Uuid,
    },
    RevenueNotFoundWithDate {
        date: NaiveDate,
    },
    RevenueNotFound {
        month: u32,
    },
    PaymentGatewayError {
        signature: String,
    },

    ProofMustContainImageOrNote,
    DuplicateProofOfDelivery,
    UpdateProofOfDeliveryError(String),
    SubmittedByEmptyError,
    ValidationError(Vec<String>),

    DuplicateEntity,
    ForeignKeyViolation,
    NullConstraintViolation,
    CheckConstraintViolation,
    DataTooLong,
    InvalidDataFormat,
    NumericOverflow,
    DeadlockDetected,
    TransactionTimeout,
    SerializationFailure,
    Internal(String),
    DatabaseError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::ShipmentNotFound { tracking_number } => {
                write!(f, "Shipment {} not found", tracking_number)
            }

            DomainError::ShipmentNotFoundById { id } => {
                write!(f, "Shipment with id: {} not found", id)
            }
            DomainError::ShipmentNotDelivered => {
                write!(f, "Shipment status found is not delivered")
            }
            DomainError::InvalidShipmentStatusTransition { from, to } => {
                write!(
                    f,
                    "Invalid shipment status transition from {} to {}",
                    from, to
                )
            }

            DomainError::InvalidPaymentStatusTransition { from, to } => {
                write!(
                    f,
                    "Invalid payment status transition from {} to {}",
                    from, to
                )
            }

            DomainError::InvalidSupportStatusTransition { from, to } => {
                write!(
                    f,
                    "Invalid support status transition from {} to {}",
                    from, to
                )
            }

            DomainError::ProofMustContainImageOrNote => write!(
                f,
                "Proof of delivery must contain either an image or a note."
            ),

            DomainError::DuplicateProofOfDelivery => write!(f, "Duplicate proof detected."),

            DomainError::UpdateProofOfDeliveryError(cause) => {
                write!(f, "Unable to update the proof of delivery: {}", cause)
            }

            DomainError::SubmittedByEmptyError => {
                write!(f, "Unable to fecth the proof's sender details")
            }
            DomainError::ValidationError(causes) => {
                write!(f, "Validation failed: {}", causes.join(", "))
            }
            DomainError::DuplicateEntity => {
                write!(f, "Duplicate entity")
            }

            DomainError::PaymentExistsForThisShipment { id } => {
                write!(f, "Payment for shipment with id: {} already made", id)
            }
            DomainError::PaymentNotFound { reference } => {
                write!(f, "Payment {} not found", reference)
            }
            DomainError::PaymentWithShipmentIdNotFound { shipment_id } => {
                write!(f, "Payment with shipment id {} not found", shipment_id)
            }
            DomainError::PaymentGatewayError { signature } => {
                write!(f, "Invalid signature {} ", signature)
            }
            DomainError::RevenueNotFoundWithDate { date } => {
                write!(f, "Revenue {} not found", date)
            }
            DomainError::RevenueNotFound { month } => {
                write!(f, "Revenue {} not found", month)
            }
            DomainError::ForeignKeyViolation => {
                write!(f, "Foreign key violation")
            }

            DomainError::NullConstraintViolation => {
                write!(f, "Null constraint violation")
            }

            DomainError::CheckConstraintViolation => {
                write!(f, "Check constraint violation")
            }

            DomainError::DataTooLong => {
                write!(f, "Data too long")
            }

            DomainError::InvalidDataFormat => {
                write!(f, "Invalid data format")
            }

            DomainError::NumericOverflow => {
                write!(f, "Numeric overflow")
            }

            DomainError::DeadlockDetected => {
                write!(f, "Deadlock detected")
            }

            DomainError::TransactionTimeout => {
                write!(f, "Transaction timeout")
            }

            DomainError::SerializationFailure => {
                write!(f, "Serialization failure")
            }

            DomainError::DatabaseError(msg) => {
                write!(f, "Database error: {}", msg)
            }
            DomainError::Internal(msg) => {
                write!(f, "Internal system error: {}", msg)
            }
        }
    }
}

// Mapping RepoError to DOmainError
impl From<RepositoryError> for DomainError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::DuplicateEntity => DomainError::DuplicateEntity,
            RepositoryError::ForeignKeyViolation => DomainError::ForeignKeyViolation,
            RepositoryError::NullConstraintViolation => DomainError::NullConstraintViolation,
            RepositoryError::CheckConstraintViolation => DomainError::CheckConstraintViolation,
            RepositoryError::DataTooLong => DomainError::DataTooLong,
            RepositoryError::InvalidDataFormat => DomainError::InvalidDataFormat,
            RepositoryError::NumericOverflow => DomainError::NumericOverflow,
            RepositoryError::DeadlockDetected => DomainError::DeadlockDetected,
            RepositoryError::TransactionTimeout => DomainError::TransactionTimeout,
            RepositoryError::SerializationFailure => DomainError::SerializationFailure,
            RepositoryError::DatabaseError(msg) => DomainError::DatabaseError(msg),
        }
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(err: serde_json::Error) -> Self {
        DomainError::DatabaseError(err.to_string())
    }
}

impl std::error::Error for DomainError {}
