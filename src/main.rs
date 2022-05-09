use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use backtrace::Backtrace;

const DEFAULT_PORT: &str = "8080";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up our panic handler
    std::panic::set_hook(Box::new(|_panic_info| {
        let backtrace = Backtrace::new();
        println!("Caught panic: {:#?}", backtrace);
        // TODO upload our backtrace to an aptly-named service?
    }));

    // Pull in config from environment variables
    let port = std::env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());

    // Set up web server and start accepting connections
    let app = move || App::new().service(echo).service(panic);
    HttpServer::new(app)
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await?;

    Ok(())
}

#[actix_web::post("/echo")]
async fn echo(_req: HttpRequest, text: String) -> impl Responder {
    HttpResponse::Ok().body(format!("You said: {text}"))
}

#[actix_web::get("/panic")]
async fn panic() -> impl Responder {
    // This panic isn't caught by us, but Actix.
    // This request fails, but Actix continues to handle requests.
    panic!("Panic!");

    HttpResponse::InternalServerError().finish()
}
