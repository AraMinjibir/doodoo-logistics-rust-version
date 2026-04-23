use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ProofOfDelivery {
    image: Option<String>,
    note: String,
    submitted_by: String,
    submitted_at: DateTime<Utc>,
}


impl ProofOfDelivery {
    pub fn create(
        image: Option<String>,
        note: String,
        submitted_by: String,
    ) -> Result<Self, Vec<DomainError>> {
        let cleaned_image = image
            .map(|img| img.trim().to_string())
            .filter(|img| !img.is_empty());

        let cleaned_note = note.trim().to_string();
        let cleaned_submitter = submitted_by.trim().to_string();

        let mut errors = Vec::new();

        if cleaned_image.is_none() && cleaned_note.is_empty() {
            errors.push(DomainError::ProofMustContainImageOrNote);
        }

        if cleaned_submitter.is_empty() {
            errors.push(DomainError::SubmittedByEmpty);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            image: cleaned_image,
            note: cleaned_note,
            submitted_by: cleaned_submitter,
            submitted_at: Utc::now(),
        })
    }
}