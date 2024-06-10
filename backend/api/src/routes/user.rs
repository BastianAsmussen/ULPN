use std::future::{ready, Ready};

use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::web::{Data, Json};
use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, FromRequest, HttpRequest,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel_async::AsyncPgConnection;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::db::models::user::{AccessLevel, NewUser, User};
use crate::routes::APIError;
use crate::state::App;

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub picture: String,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

#[post("/auth/register")]
pub async fn register_user_handler(body: Json<NewUser>, data: Data<App>) -> impl Responder {
    let mut conn = data
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let user = body.into_inner();
    let result = user.insert(&mut conn).await;

    result.map_or_else(
        |_| Err(APIError::InternalServerError),
        |user| Ok(HttpResponse::Created().json(user)),
    )
}

#[post("/auth/login")]
pub async fn login_user_handler(
    body: Json<OAuthResponse>,
    data: Data<App>,
) -> Result<HttpResponse, APIError> {
    let oauth_response = body.into_inner();

    let mut conn = data
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let Ok(user) = User::by_email(&mut conn, &oauth_response.id_token).await else {
        return Ok(HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password!"})));
    };

    let jwt_secret = &data.config().jwt_secret;
    let jaw_max_age = &data
        .config()
        .jwt_max_age
        .parse::<usize>()
        .map_err(|_| APIError::InternalServerError)?;

    let timestamp =
        usize::try_from(Utc::now().timestamp()).map_err(|_| APIError::InternalServerError)?;
    let iat = timestamp;
    let exp = timestamp + jaw_max_age * 60;

    let claims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| APIError::InternalServerError)?;

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(Duration::seconds(*jaw_max_age as i64))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"})))
}

#[get("/sessions/oauth/google")]
async fn google_oauth_handler(
    query: web::Query<QueryCode>,
    data: Data<App>,
) -> Result<impl Responder, APIError> {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return Ok(HttpResponse::Unauthorized()
            .json(json!({"status": "fail", "message": "Authorization code not provided!"})));
    }

    let Ok(token_response) = request_token(code.as_str(), &data).await else {
        return Ok(HttpResponse::BadGateway()
            .json(json!({"status": "fail", "message": "Failed to get token!"})));
    };

    let Ok(google_user) =
        get_google_user(&token_response.access_token, &token_response.id_token).await
    else {
        return Ok(HttpResponse::BadGateway()
            .json(json!({"status": "fail", "message": "Failed to get user info!"})));
    };

    let mut conn = data
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let user = if let Ok(user) = User::by_email(&mut conn, &google_user.email).await {
        user
    } else {
        let new_user = NewUser {
            email: google_user.email.clone(),
            full_name: google_user.name.clone(),
            access_level: AccessLevel::Child, // TODO: Change this to a valid access level!
        };
        new_user
            .insert(&mut conn)
            .await
            .map_err(|_| APIError::InternalServerError)?
    };

    let jaw_max_age = &data
        .config()
        .jwt_max_age
        .parse::<usize>()
        .map_err(|_| APIError::InternalServerError)?;
    let timestamp =
        usize::try_from(Utc::now().timestamp()).map_err(|_| APIError::InternalServerError)?;
    let iat = timestamp;
    let exp = timestamp + jaw_max_age * 60;

    let claims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.config().jwt_secret.as_ref()),
    )
    .map_err(|_| APIError::InternalServerError)?;

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(Duration::seconds(*jaw_max_age as i64))
        .http_only(true)
        .finish();

    let frontend_origin = &data.config().client_origin;
    let response = HttpResponse::Found()
        .append_header(("Location", format!("{frontend_origin}/{state}")))
        .cookie(cookie)
        .finish();

    Ok(response)
}

#[get("/auth/logout")]
async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(Duration::new(0, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

async fn request_token(
    authorization_code: &str,
    data: &Data<App>,
) -> Result<OAuthResponse, APIError> {
    let client_id = &data.config().google_oauth_client_id;
    let client_secret = &data.config().google_oauth_client_secret;
    let redirect_url = &data.config().google_oauth_redirect_url;

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client
        .post(root_url)
        .form(&params)
        .send()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    if response.status().is_success() {
        let oauth_response = response
            .json::<OAuthResponse>()
            .await
            .map_err(|_| APIError::InternalServerError)?;

        Ok(oauth_response)
    } else {
        Err(APIError::InternalServerError)
    }
}

async fn get_google_user(access_token: &str, id_token: &str) -> Result<GoogleUserResult, APIError> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo")
        .map_err(|_| APIError::InternalServerError)?;
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client
        .get(url)
        .bearer_auth(id_token)
        .send()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    if response.status().is_success() {
        let user_info = response
            .json::<GoogleUserResult>()
            .await
            .map_err(|_| APIError::InternalServerError)?;

        Ok(user_info)
    } else {
        Err(APIError::InternalServerError)
    }
}

pub struct AuthenticationGuard {
    pub user_id: String,
}

impl AuthenticationGuard {
    pub async fn has_access(
        &self,
        conn: &mut AsyncPgConnection,
        required_level: &AccessLevel,
    ) -> bool {
        let user = User::by_id(conn, self.user_id.parse().unwrap())
            .await
            .map_err(|_| APIError::InternalServerError)
            .unwrap();

        user.access_level >= *required_level
    }
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        let Some(token) = token else {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Unauthorized"}).to_string(),
            )));
        };

        let data = req.app_data::<Data<App>>().unwrap();
        let jwt_secret = &data.config().jwt_secret;

        let decoded = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decoded {
            Ok(token) => {
                let user_id = token.claims.sub;

                ready(Ok(Self { user_id }))
            }
            Err(_) => ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Unauthorized"}).to_string(),
            ))),
        }
    }
}
