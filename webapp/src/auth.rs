use actix_web::{get, Responder, HttpResponse, cookie::Cookie, dev::HttpServiceFactory, web::{self, Query}, HttpRequest};
use rand::Rng;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize, de::IntoDeserializer};
use serde_json::{to_vec, Deserializer};


#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    scope: String,
    expires_in: i32,
    token_type: String
}

pub fn validate(auth: BearerAuth) {

}


pub fn auth_service(area: &str) -> impl HttpServiceFactory {
    web::scope(area)
       .service(login)
       .service(auth0_callback)
}

#[get("login")]
pub async fn login() -> impl Responder {
    let state = random_state();

    let url = format!(
        "https://{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile&state={}&audience={}",
        AUTH0_DOMAIN,
        CLIENT_ID,
        urlencoding::encode(REDIRECT_URI),
        state,
        "api:web"
    );

    
    HttpResponse::Found()
        .cookie(Cookie::new("state",state))
        .append_header(("Location", url))
        .finish()

}

const AUTH0_DOMAIN : &str = "dev-ynzhftc5pt44qccp.us.auth0.com";
const CLIENT_ID : &str = "5IMuvjfrwIDYxHyyO5ErEW2PVR2P6Hww";
const REDIRECT_URI : &str = "http://localhost:8080/auth/auth0_callback";
const CLIENT_SECRET : &str = "4Odst8M-3iLNIZG-yQ0xBdIg-1dPN3FeBMH3hlk4HbMWN_KILqH46TJwiLz-xLhL";
const APP_URL : &str = "http://localhost:8080/";

#[derive(Deserialize)]
struct Auth0CallbackParams {
    code: String, 
    state: String
}

#[get("/auth0_callback")]
async fn auth0_callback(req: HttpRequest, q: Query<Auth0CallbackParams>) -> impl Responder {
    match req.cookie("state").map(|c|c.value().to_string()) {
        Some(state) if state == q.state => {} 
        Some(_) => return HttpResponse::Unauthorized().body("Incorrect state cookie"),
        None => return HttpResponse::BadRequest().body("No State cookie"),
    };

    let token_request = TokenRequest {
            grant_type: String::from("authorization_code"),
            client_id: CLIENT_ID.to_string(),
            client_secret: CLIENT_SECRET.to_string(),
            code: q.code.to_string(),
            redirect_uri: REDIRECT_URI.to_string(),
            audience: APP_URL.to_string()
        };


    let token_endpoint = format!("https://{}/oauth/token", AUTH0_DOMAIN);

    let client = reqwest::Client::new();
    let resp = client
        .post(&token_endpoint)
        .header("Content-Type", "application/json")
        .body(to_vec(&token_request).unwrap())
        .send()
        .await
        .unwrap()
        .text().await.unwrap();

    let token : TokenResponse = serde_json::from_str(&resp).unwrap();
        
    let mut hr =  HttpResponse::Ok().body(resp);

    hr.add_removal_cookie(&Cookie::new("state","")).unwrap();
    //hr.add_cookie(&Cookie::new("jwt", token.access_token)).unwrap();

    hr
}


#[derive(Serialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    audience: String,
    redirect_uri: String
}
fn random_state() -> String{
    use rand::{distributions::Alphanumeric, thread_rng};
    use std::iter;
    let mut rng = thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char)
        .take(7)
        .collect()
}