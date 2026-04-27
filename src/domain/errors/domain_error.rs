use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    ProofMustContainImageOrNote,
    DuplicateProofOfDelivery,
    UpdateProofOfDeliveryError(String),
    SubmittedByEmptyError,
    ValidationError(Vec<String>),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::ProofMustContainImageOrNote =>
                write!(f, "Proof of delivery must contain either an image or a note."),

            DomainError::DuplicateProofOfDelivery =>
                write!(f, "Duplicate proof detected."),

            DomainError::UpdateProofOfDeliveryError(cause) =>
                write!(f, "Unable to update the proof of delivery: {}", cause),

            DomainError::SubmittedByEmptyError  =>
            write!(f, "Unable to fecth the proof's sender details"),   
            DomainError::ValidationError(causes) =>
                write!(f, "Validation failed: {}", causes.join(", ")),
                
 
        }
    }
}

impl std::error::Error for DomainError {}