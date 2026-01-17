use std::sync::Arc;

use cookie::Cookie;
use cookie::time::Duration;
use ntex::http::header;
use ntex::web::types::{Json, State};
use ntex::web::{self, ServiceConfig, scope};

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{config_loader::get_stage, stage::Stage},
    infrastructure::jwt_authentication::authentication_model::LoginModel,
    infrastructure::postgres::repositories::{
        adventurers::AdventurersPostgres, guild_commanders::GuildCommandersPostgres,
    },
};

pub fn authentication_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/authentication")
            .service(adventurers_login)
            .service(adventurers_refresh_token)
            .service(guild_commanders_login)
            .service(guild_commanders_refresh_token),
    );
}

#[web::post("/adventurers/login")]
pub async fn adventurers_login(
    authentication_use_case: State<
        Arc<AuthenticationUseCase<AdventurersPostgres, GuildCommandersPostgres>>,
    >,
    login_model: Json<LoginModel>,
) -> web::HttpResponse {
    match authentication_use_case
        .adventurers_login(login_model.into_inner())
        .await
    {
        Ok(passport) => {
            let mut response = web::HttpResponse::Ok().body("Login successfully");

            let act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                let secure_act_cookie = act_cookie.secure(true);
                let secure_rft_cookie = rft_cookie.secure(true);
                response.headers_mut().append(
                    header::SET_COOKIE,
                    secure_act_cookie.to_string().parse().unwrap(),
                );
                response.headers_mut().append(
                    header::SET_COOKIE,
                    secure_rft_cookie.to_string().parse().unwrap(),
                );
            } else {
                response
                    .headers_mut()
                    .append(header::SET_COOKIE, act_cookie.to_string().parse().unwrap());
                response
                    .headers_mut()
                    .append(header::SET_COOKIE, rft_cookie.to_string().parse().unwrap());
            }

            response
        }
        Err(e) => web::HttpResponse::Unauthorized().body(e.to_string()),
    }
}

#[web::post("/adventurers/refresh-token")]
pub async fn adventurers_refresh_token(
    authentication_use_case: State<
        Arc<AuthenticationUseCase<AdventurersPostgres, GuildCommandersPostgres>>,
    >,
    req: web::HttpRequest,
) -> web::HttpResponse {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(refresh_token) = get_cookie_value(cookie_str, "rft") {
                match authentication_use_case
                    .adventurers_refresh_token(refresh_token)
                    .await
                {
                    Ok(passport) => {
                        let mut response =
                            web::HttpResponse::Ok().body("Refresh token successfully");

                        let act_cookie = Cookie::build(("act", passport.access_token.clone()))
                            .path("/")
                            .same_site(cookie::SameSite::Lax)
                            .http_only(true)
                            .max_age(Duration::days(14));

                        let rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                            .path("/")
                            .same_site(cookie::SameSite::Lax)
                            .http_only(true)
                            .max_age(Duration::days(14));

                        if get_stage() == Stage::Production {
                            let secure_act_cookie = act_cookie.secure(true);
                            let secure_rft_cookie = rft_cookie.secure(true);
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                secure_act_cookie.to_string().parse().unwrap(),
                            );
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                secure_rft_cookie.to_string().parse().unwrap(),
                            );
                        } else {
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                act_cookie.to_string().parse().unwrap(),
                            );
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                rft_cookie.to_string().parse().unwrap(),
                            );
                        }

                        return response;
                    }
                    Err(e) => return web::HttpResponse::Unauthorized().body(e.to_string()),
                }
            }
        }
    }

    web::HttpResponse::BadRequest().body("Refresh token not found")
}

#[web::post("/guild-commanders/login")]
pub async fn guild_commanders_login(
    authentication_use_case: State<
        Arc<AuthenticationUseCase<AdventurersPostgres, GuildCommandersPostgres>>,
    >,
    login_model: Json<LoginModel>,
) -> web::HttpResponse {
    match authentication_use_case
        .guild_commanders_login(login_model.into_inner())
        .await
    {
        Ok(passport) => {
            let mut response = web::HttpResponse::Ok().body("Login successfully");

            let act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                let secure_act_cookie = act_cookie.secure(true);
                let secure_rft_cookie = rft_cookie.secure(true);
                response.headers_mut().append(
                    header::SET_COOKIE,
                    secure_act_cookie.to_string().parse().unwrap(),
                );
                response.headers_mut().append(
                    header::SET_COOKIE,
                    secure_rft_cookie.to_string().parse().unwrap(),
                );
            } else {
                response
                    .headers_mut()
                    .append(header::SET_COOKIE, act_cookie.to_string().parse().unwrap());
                response
                    .headers_mut()
                    .append(header::SET_COOKIE, rft_cookie.to_string().parse().unwrap());
            }

            response
        }
        Err(e) => web::HttpResponse::Unauthorized().body(e.to_string()),
    }
}

#[web::post("/guild-commanders/refresh-token")]
pub async fn guild_commanders_refresh_token(
    authentication_use_case: State<
        Arc<AuthenticationUseCase<AdventurersPostgres, GuildCommandersPostgres>>,
    >,
    req: web::HttpRequest,
) -> web::HttpResponse {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            if let Some(refresh_token) = get_cookie_value(cookie_str, "rft") {
                match authentication_use_case
                    .guild_commanders_refresh_token(refresh_token)
                    .await
                {
                    Ok(passport) => {
                        let mut response =
                            web::HttpResponse::Ok().body("Refresh token successfully");

                        let act_cookie = Cookie::build(("act", passport.access_token.clone()))
                            .path("/")
                            .same_site(cookie::SameSite::Lax)
                            .http_only(true)
                            .max_age(Duration::days(14));

                        let rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                            .path("/")
                            .same_site(cookie::SameSite::Lax)
                            .http_only(true)
                            .max_age(Duration::days(14));

                        if get_stage() == Stage::Production {
                            let secure_act_cookie = act_cookie.secure(true);
                            let secure_rft_cookie = rft_cookie.secure(true);
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                secure_act_cookie.to_string().parse().unwrap(),
                            );
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                secure_rft_cookie.to_string().parse().unwrap(),
                            );
                        } else {
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                act_cookie.to_string().parse().unwrap(),
                            );
                            response.headers_mut().append(
                                header::SET_COOKIE,
                                rft_cookie.to_string().parse().unwrap(),
                            );
                        }

                        return response;
                    }
                    Err(e) => return web::HttpResponse::Unauthorized().body(e.to_string()),
                }
            }
        }
    }

    web::HttpResponse::BadRequest().body("Refresh token not found")
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
