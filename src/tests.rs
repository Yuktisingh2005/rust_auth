#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use diesel::r2d2::{self, ConnectionManager};
    use dotenv::dotenv;

    type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

    #[actix_web::test]
    async fn test_signup() {
        let app = test::init_service(App::new().service(signup)).await;
        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&SignupRequest {
                email: String::from("test@example.com"),
                password: String::from("password123"),
            })
            .to_request();

        let resp: HttpResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), HttpStatus::Created);
    }

    #[actix_web::test]
    async fn test_login() {
        let app = test::init_service(App::new().service(login)).await;
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&LoginRequest {
                email: String::from("test@example.com"),
                password: String::from("password123"),
            })
            .to_request();

        let resp: HttpResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), HttpStatus::Ok);
    }
}

