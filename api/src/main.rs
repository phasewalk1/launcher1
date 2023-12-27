use actix_cors::Cors;
use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{self, Json, Query},
    App, HttpResponse, HttpServer, Responder,
};
use http::StatusCode;
use l1_models::steam::{
    InitiateAuthRequest, RedirectUrl, SteamCallbackRequest, SteamCallbackResponse,
};

mod platform;

use platform::steam::{
    error::Error as SteamError,
    redirect::SteamRedirect,
    verifier::{SteamLoginData, Verifier as SteamVerifier},
};

#[get("/steam/login")]
async fn steam_login() -> impl Responder {
    log::debug!("Steam login initiated");

    let redirector = SteamRedirect::new("http://localhost:8080", "/steam-callback").unwrap();
    let redirect_url = redirector.url;
    log::debug!("Got redirect url: {}", redirect_url.to_string());

    HttpResponse::Ok().body(redirect_url.to_string())
}

#[get("/steam-callback")]
async fn steam_callback(req: Query<SteamLoginData>) -> Result<HttpResponse, actix_web::Error> {
    log::debug!("Steam callback initiated");

    let qs = serde_urlencoded::to_string(&req.into_inner()).unwrap();

    log::debug!("Got query params from Steam!");
    log::debug!("Query string: {}", qs);

    match SteamVerifier::make_verify_request_async(&reqwest::Client::new(), &qs).await {
        Ok(steam_id) => {
            log::info!("Steam login successful!");
            log::debug!("Steam id: {}", steam_id);
            let redirect_url = format!(
                "your-custom-protocol://localhost:3000/steam/login-success?steamId={}",
                steam_id
            );
            return Ok(HttpResponse::Found()
                .header("Location", redirect_url)
                .finish());
        }
        Err(err) => {
            log::error!("Steam login failed: {}", err);
            return Err(ErrorInternalServerError(err));
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init().ok();

    log::info!("Starting server on port: 8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_methods(vec!["GET", "POST"]);
        App::new()
            .wrap(cors)
            .service(steam_login)
            .service(steam_callback)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
