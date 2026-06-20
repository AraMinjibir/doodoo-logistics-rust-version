use serde::Deserialize;
use uuid::Uuid;

use crate::{
    controllers::helpers::result_mapper::IntoDomain,
    domain::{
        errors::domain_error::DomainError,
        models::support::{Comment, Complaint},
    },
};

#[derive(Debug, Deserialize)]
pub struct ComplaintDto {
    user_id: Uuid,
    shipment_id: Uuid,
    subject: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateComplaintStatusDto {
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct CommentDto {
    author_id: Uuid,
    message: String,
}

impl IntoDomain<Complaint> for ComplaintDto {
    fn to_domain(self) -> Result<Complaint, DomainError> {
        Complaint::send_complaint(
            self.user_id,
            self.shipment_id,
            self.subject,
            self.description,
        )
    }
}

impl IntoDomain<Comment> for CommentDto {
    fn to_domain(self) -> Result<Comment, DomainError> {
        Comment::make_comment(self.author_id, self.message)
    }
}
