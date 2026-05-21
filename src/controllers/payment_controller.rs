use actix_web::{HttpRequest, HttpResponse, Responder, web};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::config::app_state::AppState;
use crate::controllers::dto::{GeneratePaymentDto, PaymentResponseDto};
use crate::controllers::helpers::result_mapper::map_domain_error;
use crate::domain::gateways::payment_gateway::PaymentWebhookEvent;


pub async fn generate_payment(
    state: web::Data<AppState>,
    payload: web::Json<GeneratePaymentDto>,
) -> impl Responder {
    let domain = match payload.into_inner().to_domain() {
        Ok(payment) => payment,
        Err(err) => return map_domain_error(err),
    };

    match state.payment_service.generate_payment(&domain).await {
        Ok(payment) => HttpResponse::Created().json(PaymentResponseDto::from_domain(payment)),

        Err(err) => map_domain_error(err),
    }
}

pub async fn get_payment_by_ref(
    state: web::Data<AppState>,
    reference: web::Path<String>,
) -> impl Responder {
    let reference_number = reference.into_inner();

    match state
        .payment_service
        .get_payment_by_ref(&reference_number)
        .await
    {
        Ok(payment) => HttpResponse::Ok().json(PaymentResponseDto::from_domain(payment)),
        Err(err) => map_domain_error(err),
    }
}

pub async fn get_payment_by_shipment_id(
    state: web::Data<AppState>,
    shipment_id: web::Path<Uuid>,
) -> impl Responder {
    let extracted_shipment_id = shipment_id.into_inner();

    match state
        .payment_service
        .get_payment_by_shipment_id(extracted_shipment_id)
        .await
    {
        Ok(payment) => HttpResponse::Ok().json(PaymentResponseDto::from_domain(payment)),
        Err(err) => map_domain_error(err),
    }
}
pub async fn get_payment_by_status(
    state: web::Data<AppState>,
    status: web::Path<String>,
) -> impl Responder {
    let payment_status = status.into_inner();

    match state
        .payment_service
        .get_payment_by_status(&payment_status)
        .await
    {
        Ok(list_payment) => {
            let payment_response: Vec<PaymentResponseDto> = list_payment
                .into_iter()
                .map(PaymentResponseDto::from_domain)
                .collect();
            HttpResponse::Ok().json(payment_response)
        }
        Err(err) => map_domain_error(err),
    }
}
pub async fn get_all_payments(state: web::Data<AppState>) -> impl Responder {
    match state.payment_service.get_all_payments().await {
        Ok(list_payments) => {
            let fetched_payments: Vec<PaymentResponseDto> = list_payments
                .into_iter()
                .map(PaymentResponseDto::from_domain)
                .collect();
            HttpResponse::Ok().json(fetched_payments)
        }
        Err(err) => map_domain_error(err),
    }
}
pub async fn get_daily_revenue(
    state: web::Data<AppState>,
    date: web::Path<NaiveDate>,
) -> impl Responder {
    let actual_date = date.into_inner();

    match state.payment_service.get_daily_revenue(actual_date).await {
        Ok(revenue) => HttpResponse::Ok().json(revenue),
        Err(err) => map_domain_error(err),
    }
}

pub async fn get_weekly_revenue(
    state: web::Data<AppState>,
    date: web::Path<NaiveDate>,
) -> impl Responder {
    let actual_date = date.into_inner();

    match state.payment_service.get_weekly_revenue(actual_date).await {
        Ok(revenue) => HttpResponse::Ok().json(revenue),
        Err(err) => map_domain_error(err),
    }
}

pub async fn get_monthly_revenue(
    state: web::Data<AppState>,
    path: web::Path<(u32, u32)>,
) -> impl Responder {
    let (month, year) = path.into_inner();

    match state.payment_service.get_monthly_revenue(year, month).await {
        Ok(revenue) => HttpResponse::Ok().json(revenue),
        Err(err) => map_domain_error(err),
    }
}

pub async fn handle_webhook(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<PaymentWebhookEvent>,
) -> impl Responder {

    // Extract signature from headers
    let signature = match req.headers().get("x-signature") {
        Some(value) => match value.to_str() {
            Ok(v) => v,
            Err(_) => {
                return HttpResponse::BadRequest().body("Invalid signature header");
            }
        },
        None => {
            return HttpResponse::BadRequest().body("Missing signature header");
        }
    };

    match state
        .payment_service
        .handle_webhook(&body.into_inner(), signature)
        .await
    {
        Ok(payment) => HttpResponse::Ok().json(PaymentResponseDto::from_domain(payment)),
        Err(err) => map_domain_error(err),
    }
}

pub async fn delete_payment(
    state: web::Data<AppState>,
    reference: web::Path<String>,
) -> impl Responder {
    let payment_ref = reference.into_inner();

    match state.payment_service.delete_payment(&payment_ref).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(err) => map_domain_error(err),
    }
}
