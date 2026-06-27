use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    config::app_state::AppState,
    controllers::{
        dto::{
            LoginDto, LoginResponse, SignUp, SignUpResponse, UpdateUserDto, UpdateUserStatusDto,
        },
        helpers::result_mapper::{map_domain_error, parse_user_status},
    },
};

pub async fn sign_up(state: web::Data<AppState>, payload: web::Json<SignUp>) -> impl Responder {
    let user = payload.into_inner().into_domain();

    match state.user_service.register_user(user).await {
        Ok(user) => HttpResponse::Created().json(SignUpResponse::from_domain(user)),
        Err(e) => map_domain_error(e),
    }
}

pub async fn login(state: web::Data<AppState>, payload: web::Json<LoginDto>) -> impl Responder {
    let LoginDto { email, password } = payload.into_inner();

    match state.user_service.login(email, password).await {
        Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
        Err(e) => map_domain_error(e),
    }
}
pub async fn get_user_by_id(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();

    match state.user_service.get_by_id(user_id).await {
        Ok(user) => HttpResponse::Ok().json(SignUpResponse::from_domain(user)),
        Err(e) => map_domain_error(e),
    }
}
pub async fn get_user_by_email(
    state: web::Data<AppState>,
    email: web::Path<String>,
) -> impl Responder {
    let email = email.into_inner();
    match state.user_service.get_by_email(&email).await {
        Ok(user) => HttpResponse::Ok().json(SignUpResponse::from_domain(user)),

        Err(e) => map_domain_error(e),
    }
}
pub async fn get_user_by_status(
    state: web::Data<AppState>,
    status: web::Path<String>,
) -> impl Responder {
    let user_status = status.into_inner();

    match state.user_service.get_by_status(&user_status).await {
        Ok(users) => {
            let fetched_users: Vec<SignUpResponse> =
                users.into_iter().map(SignUpResponse::from_domain).collect();
            HttpResponse::Ok().json(fetched_users)
        }
        Err(e) => map_domain_error(e),
    }
}
pub async fn get_user_by_role(
    state: web::Data<AppState>,
    role: web::Path<String>,
) -> impl Responder {
    let user_role = role.into_inner();

    match state.user_service.get_by_role(&user_role).await {
        Ok(users) => {
            let fetched_users: Vec<SignUpResponse> =
                users.into_iter().map(SignUpResponse::from_domain).collect();
            HttpResponse::Ok().json(fetched_users)
        }
        Err(e) => map_domain_error(e),
    }
}

pub async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    match state.user_service.get_all().await {
        Ok(fetch_users) => {
            let fetched_users: Vec<SignUpResponse> = fetch_users
                .into_iter()
                .map(SignUpResponse::from_domain)
                .collect();
            HttpResponse::Ok().json(fetched_users)
        }
        Err(e) => map_domain_error(e),
    }
}
pub async fn update_status(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateUserStatusDto>,
) -> impl Responder {
    let user_id = id.into_inner();
    let status = match parse_user_status(payload.into_inner().status) {
        Ok(user) => user,
        Err(resp) => return resp,
    };

    match state.user_service.update_status(user_id, status).await {
        Ok(updated) => HttpResponse::Ok().json(SignUpResponse::from_domain(updated)),
        Err(e) => map_domain_error(e),
    }
}

pub async fn update_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateUserDto>,
) -> impl Responder {
    let user_id = id.into_inner();
    let domain = payload.into_inner().into_input();
    match state.user_service.update_user(user_id, domain).await {
        Ok(updated) => HttpResponse::Ok().json(SignUpResponse::from_domain(updated)),
        Err(e) => map_domain_error(e),
    }
}

pub async fn delete_user(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();

    match state.user_service.delete(user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => map_domain_error(e),
    }
}
