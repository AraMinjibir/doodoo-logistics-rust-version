use crate::controllers::{
    health_controller, payment_controller, shipment_controller, support_controller, user_controller,
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_controller::health));

    cfg.service(
        web::scope("/shipments")
            .route("", web::post().to(shipment_controller::create_shipment))
            .route(
                "/{shipment_id}/provider/{provider_id}",
                web::post().to(shipment_controller::assign_service_provider),
            )
            .route("", web::get().to(shipment_controller::list_shipments))
            .route("/{id}", web::get().to(shipment_controller::get_by_id))
            .route("/{id}", web::put().to(shipment_controller::update_shipment))
            .route(
                "/{id}",
                web::delete().to(shipment_controller::delete_shipment),
            )
            .route(
                "/tracking/{tracking}",
                web::get().to(shipment_controller::get_by_tracking),
            )
            .route(
                "/tracking/{tracking}/status",
                web::patch().to(shipment_controller::update_status),
            )
            .route(
                "/provider/{provider_id}",
                web::get().to(shipment_controller::get_shipment_by_assinged_provider),
            )
            .route(
                "/tracking/{tracking}/proof",
                web::post().to(shipment_controller::upload_proof),
            )
            .route(
                "/status/{status}",
                web::get().to(shipment_controller::get_by_status),
            ),
    );

    cfg.service(
        web::scope("/payments")
            .route("", web::post().to(payment_controller::generate_payment))
            .route("", web::get().to(payment_controller::get_all_payments))
            // payment lookup
            .route(
                "/reference/{ref}",
                web::get().to(payment_controller::get_payment_by_ref),
            )
            .route(
                "/shipment/{shipment_id}",
                web::get().to(payment_controller::get_payment_by_shipment_id),
            )
            // status filtering
            .route(
                "/status/{status}",
                web::get().to(payment_controller::get_payment_by_status),
            )
            // revenue
            .route(
                "/revenue/daily/{date}",
                web::get().to(payment_controller::get_daily_revenue),
            )
            .route(
                "/revenue/weekly/{date}",
                web::get().to(payment_controller::get_weekly_revenue),
            )
            .route(
                "/revenue/monthly/{month}/{year}",
                web::get().to(payment_controller::get_monthly_revenue),
            )
            // webhook
            .route(
                "/webhook",
                web::post().to(payment_controller::handle_webhook),
            )
            // delete
            .route(
                "/{ref}",
                web::delete().to(payment_controller::delete_payment),
            ),
    );

    cfg.service(
        web::scope("/complaints")
            .route("", web::post().to(support_controller::send_complaint))
            .route("", web::get().to(support_controller::get_all_compalints))
            .route(
                "/{id}",
                web::get().to(support_controller::get_complaint_by_id),
            )
            .route(
                "/{id}",
                web::delete().to(support_controller::delete_complaint),
            )
            .route(
                "/status/{status}",
                web::get().to(support_controller::get_complaint_by_status),
            )
            .route(
                "/{id}/comment",
                web::patch().to(support_controller::make_comment),
            )
            .route("", web::post().to(support_controller::send_complaint))
            .route(
                "/{id}/status",
                web::patch().to(support_controller::update_complaint_status),
            ),
    );

    cfg.service(
        web::scope("/users")
            // Authentication
            .route("/signup", web::post().to(user_controller::sign_up))
            .route("/login", web::post().to(user_controller::login))
            // Queries
            .route("", web::get().to(user_controller::get_all_users))
            .route("/{id}", web::get().to(user_controller::get_user_by_id))
            .route(
                "/email/{email}",
                web::get().to(user_controller::get_user_by_email),
            )
            .route(
                "/status/{status}",
                web::get().to(user_controller::get_user_by_status),
            )
            .route(
                "/role/{role}",
                web::get().to(user_controller::get_user_by_role),
            )
            // Commands
            .route("/{id}", web::put().to(user_controller::update_user))
            .route(
                "/{id}/status",
                web::patch().to(user_controller::update_status),
            )
            .route("/{id}", web::delete().to(user_controller::delete_user)),
    );
}
