use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    config::app_state::AppState,
    controllers::{
        dto::{CommentDto, ComplaintDto, ComplaintResponse, UpdateComplaintStatusDto},
        helpers::result_mapper::{
            log_and_map, map_domain_error, parse_complaint_status, parse_dto,
        },
    },
};

pub async fn send_complaint(
    state: web::Data<AppState>,
    payload: web::Json<ComplaintDto>,
) -> impl Responder {
    let domain = match parse_dto(payload.into_inner()) {
        Ok(c) => c,
        Err(res) => return res,
    };

    match state.support_service.send_complaint(&domain).await {
        Ok(response) => HttpResponse::Created().json(ComplaintResponse::new(response)),
        Err(err) => map_domain_error(err),
    }
}

pub async fn make_comment(
    state: web::Data<AppState>,
    complaint_id: web::Path<Uuid>,
    comment: web::Json<CommentDto>,
) -> impl Responder {
    let id = complaint_id.into_inner();
    let domain = match parse_dto(comment.into_inner()) {
        Ok(comment) => comment,
        Err(res) => return res,
    };

    match state.support_service.send_comment(id, domain).await {
        Ok(response) => HttpResponse::Created().json(ComplaintResponse::new(response)),
        Err(err) => map_domain_error(err),
    }
}

pub async fn get_complaint_by_id(
    state: web::Data<AppState>,
    complaint_id: web::Path<Uuid>,
) -> impl Responder {
    let id = complaint_id.into_inner();

    match state.support_service.get_complaint_by_id(id).await {
        Ok(complaint) => HttpResponse::Ok().json(ComplaintResponse::new(complaint)),
        Err(err) => log_and_map(err),
    }
}

pub async fn get_complaint_by_status(
    state: web::Data<AppState>,
    complaint_status: web::Path<String>,
) -> impl Responder {
    let status = match parse_complaint_status(complaint_status.into_inner()) {
        Ok(s) => s,
        Err(resp) => return resp,
    };
    match state.support_service.get_complaint_by_status(&status).await {
        Ok(complaints) => {
            let response: Vec<ComplaintResponse> =
                complaints.into_iter().map(ComplaintResponse::new).collect();

            HttpResponse::Ok().json(response)
        }
        Err(e) => log_and_map(e),
    }
}

pub async fn get_all_compalints(state: web::Data<AppState>) -> impl Responder {
    match state.support_service.get_all_compalints().await {
        Ok(all_compalaints) => {
            let res: Vec<ComplaintResponse> = all_compalaints
                .into_iter()
                .map(ComplaintResponse::new)
                .collect();
            HttpResponse::Ok().json(res)
        }
        Err(e) => log_and_map(e),
    }
}

pub async fn update_complaint_status(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateComplaintStatusDto>,
) -> impl Responder {
    let id = id.into_inner();
    let status = match parse_complaint_status(payload.into_inner().status) {
        Ok(status) => status,
        Err(res) => return res,
    };

    match state
        .support_service
        .update_complaint_status(id, &status)
        .await
    {
        Ok(updated) => HttpResponse::Ok().json(ComplaintResponse::new(updated)),
        Err(err) => map_domain_error(err),
    }
}

pub async fn delete_complaint(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();
    match state.support_service.delete_complaint(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => log_and_map(err),
    }
}
