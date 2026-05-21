use actix_web::{HttpResponse};
use serde_json::json;


use crate::domain::errors::domain_error::DomainError;
use crate::domain::models::shipment_status::ShipmentStatus;


pub fn map_domain_error(err: DomainError) -> HttpResponse {
    match err {
        // NOT FOUND
        DomainError::ShipmentNotFound { tracking_number } => {
            HttpResponse::NotFound().json(json!({
                "error": "ShipmentNotFound",
                "message": format!("Shipment {} not found", tracking_number)
            }))
        }

        DomainError::ShipmentNotFoundById { id } => {
            HttpResponse::NotFound().json(json!({
                "error": "ShipmentNotFound",
                "message": format!("Shipment with id {} not found", id)
            }))
        }
        DomainError::PaymentNotFound {reference } => {
            HttpResponse::NotFound().json(json!({
                "error": "PaymentNotFound",
                "message": format!("Payment {} not found", reference)
            }))
        }
        
        DomainError::PaymentWithShipmentIdNotFound {shipment_id } => {
            HttpResponse::NotFound().json(json!({
                "error": "PaymentNotFound",
                "message": format!("Payment with id: {} not found", shipment_id)
            }))
        }
        DomainError::RevenueNotFoundWithDate {date } => {
            HttpResponse::NotFound().json(json!({
                "error": "RevenueNotFound",
                "message": format!("Revenue for this day: {} not found", date)
            }))
        }
        DomainError::RevenueNotFound {month } => {
            HttpResponse::NotFound().json(json!({
                "error": "RevenueNotFound",
                "message": format!("Revenue for this month: {} not found", month)
            }))
        }

        // VALIDATION / CLIENT ERRORS
        DomainError::ValidationError(errors) => {
            HttpResponse::BadRequest().json(json!({
                "error": "ValidationError",
                "messages": errors
            }))
        }

        DomainError::InvalidShipmentStatusTransition { from, to } => {
            HttpResponse::BadRequest().json(json!({
                "error": "InvalidShipmentStatusTransition",
                "message": format!("Cannot move from {:?} to {:?}", from, to)
            }))
        }

        DomainError::ShipmentNotDelivered => {
            HttpResponse::BadRequest().json(json!({
                "error": "ShipmentNotDelivered",
                "message": "Shipment must be delivered before this operation"
            }))
        }

        DomainError::ProofMustContainImageOrNote => {
            HttpResponse::BadRequest().json(json!({
                "error": "ProofValidationError",
                "message": "Proof must contain either an image or a note"
            }))
        }

        DomainError::SubmittedByEmptyError => {
            HttpResponse::BadRequest().json(json!({
                "error": "SubmittedByEmpty",
                "message": "Submitted_by cannot be empty"
            }))
        }

        DomainError::PaymentGatewayError{signature} => {
            HttpResponse::BadRequest().json(json!({
                "error": "ShipmentNotDelivered",
                "message": format!("Invalid signature {} ", signature)
            }))
        }
        // CONFLICT (BUSINESS RULES)
        DomainError::DuplicateProofOfDelivery => {
            HttpResponse::Conflict().json(json!({
                "error": "DuplicateProof",
                "message": "Proof of delivery already exists"
            }))
        }

        DomainError::DuplicateEntity => {
            HttpResponse::Conflict().json(json!({
                "error": "DuplicateEntity",
                "message": "Entity already exists"
            }))
        }
        DomainError::PaymentExistsForThisShipment {id } => {
            HttpResponse::Conflict().json(json!({
                "error": "Duplicate",
                "message": format!("Payment with shipment id: {}, already exists", id)
            }))
        }

        // DATABASE / INFRASTRUCTURE
        DomainError::ForeignKeyViolation
        | DomainError::NullConstraintViolation
        | DomainError::CheckConstraintViolation
        | DomainError::DataTooLong
        | DomainError::InvalidDataFormat
        | DomainError::NumericOverflow => {
            HttpResponse::BadRequest().json(json!({
                "error": "DatabaseConstraintViolation",
                "message": err.to_string()
            }))
        }

        DomainError::DeadlockDetected
        | DomainError::TransactionTimeout
        | DomainError::SerializationFailure => {
            HttpResponse::ServiceUnavailable().json(json!({
                "error": "TransientDatabaseError",
                "message": "Temporary database issue, please retry"
            }))
        }

        DomainError::DatabaseError(msg) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "DatabaseError",
                "message": msg
            }))
        }

        // FALLBACK
        _ => {
            HttpResponse::InternalServerError().json(json!({
                "error": "InternalError",
                "message": err.to_string()
            }))
        }
    }
}

pub fn extract_or_bad_request<T>(
    res: Result<T, DomainError>,
) -> Result<T, HttpResponse> {
    res.map_err(map_domain_error)
}

pub trait IntoDomain<D> {
    fn to_domain(self) -> Result<D, DomainError>;
}

pub fn parse_dto<T, D>(dto: T) -> Result<D, HttpResponse>
where
    T: IntoDomain<D>,
{
    extract_or_bad_request(dto.to_domain())
}

pub fn parse_status(input: String) -> Result<ShipmentStatus, HttpResponse> {
    input
        .parse()
        .map_err(|_| HttpResponse::BadRequest().body("Invalid status"))
}

pub fn log_and_map(e: DomainError) -> HttpResponse {
    tracing::error!(error = ?e, "Shipment error");
    map_domain_error(e)
}

