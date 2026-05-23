use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;


use crate::config::app_state::AppState;
use crate::controllers::dto::{
    CreateShipmentDto, 
    ShipmentResponseDto,
    PaginationQuery, 
    ProofOfDeliveryDto, 
    UpdateShipmentDto,
    UpdateStatusDto};
use crate::controllers::helpers::result_mapper::
{map_domain_error, extract_or_bad_request,parse_dto, parse_status, log_and_map};


pub async fn create_shipment(
    state: web::Data<AppState>,
    payload: web::Json<CreateShipmentDto>,
) -> impl Responder {

    let domain = match parse_dto(payload.into_inner()) {
    Ok(d) => d,
    Err(resp) => return resp,
};

    match state.shipment_service.create_shipment(domain).await {
        Ok(shipment) => HttpResponse::Created().json(ShipmentResponseDto::from(shipment)),
        Err(e) => log_and_map(e)
    }
}

pub async fn get_by_tracking(
    state: web::Data<AppState>,
    tracking: web::Path<String>,
) -> impl Responder {
    match state
        .shipment_service
        .get_by_tracking_number(&tracking)
        .await
    {
        Ok(shipment) => HttpResponse::Ok().json(ShipmentResponseDto::from(shipment)),
        Err(e) => log_and_map(e)
    }
}

pub async fn get_by_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match state.shipment_service.get_by_id(id.into_inner()).await {
        Ok(shipment) => {
            HttpResponse::Ok().json(ShipmentResponseDto::from(shipment))
        }

        Err(e) => log_and_map(e),
    }
}

pub async fn get_by_status(
    state: web::Data<AppState>,
    status: web::Path<String>,
) -> impl Responder {
    let status = match parse_status(status.into_inner()) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    match state.shipment_service.get_by_status(status).await {
        Ok(list) => {
            let response: Vec<ShipmentResponseDto> =
                list.into_iter().map(ShipmentResponseDto::from).collect();

            HttpResponse::Ok().json(response)
        }
        Err(e) => log_and_map(e)
    }
}

pub async fn list_shipments(
    state: web::Data<AppState>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let page = query.page.max(1);
    let page_size = query.page_size.clamp(1, 100);

    let offset = (page - 1) * page_size;

    match state
        .shipment_service
        .list_shipments(offset as i64, page_size as i64)
        .await
    {
        Ok(shipments) => {
            let response: Vec<ShipmentResponseDto> =
                shipments.into_iter().map(ShipmentResponseDto::from).collect();

            HttpResponse::Ok().json(response)
        }
        Err(e) => log_and_map(e)
    }
}

pub async fn update_status(
    state: web::Data<AppState>,
    tracking: web::Path<String>,
    payload: web::Json<UpdateStatusDto>,
) -> impl Responder {
    let status = match parse_status(payload.into_inner().status) {
        Ok(s) => s,
        Err(resp) => return resp,
    };

    match state
        .shipment_service
        .update_status(&tracking, status, None)
        .await
    {
        Ok(updated) => HttpResponse::Ok().json(ShipmentResponseDto::from(updated)),
        Err(e) => log_and_map(e)
    }
}

pub async fn update_shipment(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateShipmentDto>,
) -> impl Responder {

    let cmd = match payload.into_inner().into_command() {
        Ok(c) => c,
        Err(e) => return log_and_map(e),
    };

    match state
        .shipment_service
        .update_shipment(id.into_inner(), cmd)
        .await
    {
        Ok(updated) => HttpResponse::Ok().json(ShipmentResponseDto::from(updated)),
        Err(e) => log_and_map(e),
    }
}

pub async fn upload_proof(
    state: web::Data<AppState>,
    tracking: web::Path<String>,
    payload: web::Json<ProofOfDeliveryDto>,
) -> impl Responder {
    let proof = match extract_or_bad_request(payload.into_inner().to_domain()) {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    match state
        .shipment_service
        .upload_proof_of_delivery(&tracking, proof)
        .await
    {
        Ok(updated) => HttpResponse::Ok().json(ShipmentResponseDto::from(updated)),
        Err(e) => {
            tracing::error!("Shipment error: {:?}", e);
            map_domain_error(e)
        },
    }
}

pub async fn delete_shipment(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match state.shipment_service.delete_shipment(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => log_and_map(e)
    }
}



