use ntex::http::header;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;

use crate::{
    config::config_loader::{get_adventurers_secret_env, get_guild_commanders_secret_env},
    infrastructure::jwt_authentication,
};

// Guild Commanders Authorization Middleware
pub struct GuildCommandersAuthorization;

impl<S> Middleware<S> for GuildCommandersAuthorization {
    type Service = GuildCommandersAuthorizationMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        GuildCommandersAuthorizationMiddleware { service }
    }
}

pub struct GuildCommandersAuthorizationMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for GuildCommandersAuthorizationMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        if let Some(cookie_header) = req.headers().get(header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                let access_token = get_cookie_value(cookie_str, "act");

                if let Some(token) = access_token {
                    if let Ok(secret_env) = get_guild_commanders_secret_env() {
                        if let Ok(claims) =
                            jwt_authentication::verify_token(secret_env.secret.to_string(), token)
                        {
                            if let Ok(guild_commander_id) = claims.sub.parse::<i32>() {
                                req.extensions_mut().insert(guild_commander_id);
                                return ctx.call(&self.service, req).await;
                            }
                        }
                    }
                }
            }
        }

        let (http_req, _payload) = req.into_parts();

        Ok(web::WebResponse::new(
            web::HttpResponse::Unauthorized().finish(),
            http_req,
        ))
    }
}

// Adventurers Authorization Middleware
pub struct AdventurersAuthorization;

impl<S> Middleware<S> for AdventurersAuthorization {
    type Service = AdventurersAuthorizationMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        AdventurersAuthorizationMiddleware { service }
    }
}

pub struct AdventurersAuthorizationMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for AdventurersAuthorizationMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        if let Some(cookie_header) = req.headers().get(header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                if let Some(token) = get_cookie_value(cookie_str, "act") {
                    if let Ok(secret_env) = get_adventurers_secret_env() {
                        if let Ok(claims) =
                            jwt_authentication::verify_token(secret_env.secret.to_string(), token)
                        {
                            if let Ok(adventurer_id) = claims.sub.parse::<i32>() {
                                req.extensions_mut().insert(adventurer_id);
                                return ctx.call(&self.service, req).await;
                            }
                        }
                    }
                }
            }
        }

        let (http_req, _payload) = req.into_parts();

        Ok(web::WebResponse::new(
            web::HttpResponse::Unauthorized().finish(),
            http_req,
        ))
    }
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
