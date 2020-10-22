#[macro_use]
extern crate serde_derive;

use actix_web::{get, web, App, HttpServer, Result};

#[derive(Serialize, Deserialize)]
struct Status {
    online: bool,
}

#[get("/")]
async fn home() -> Result<web::Json<Status>> {
    let response = Status {
        online: true
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .service(home),
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: Status = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.online, true);
    }
}