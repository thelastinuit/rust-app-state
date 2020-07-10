use actix_web::{
  web,
  App,
  guard,
  HttpServer
};
use std::sync::Mutex;

struct AppState {
  app_name: String,
  counter: Mutex<i32>,
}

fn api_configuration(config: &mut web::ServiceConfig) {
  config.service(web::resource("/hello")
    .guard(guard::Header("Host", "api.hello-rust.org"))
    .route(web::get().to(hello)));
}

fn main_configuration(config: &mut web::ServiceConfig) {
  config.service(web::resource("/hello")
    .guard(guard::Header("Host", "main.hello-rust.org"))
    .route(web::get().to(hello)));
}

async fn hello(data: web::Data<AppState>) -> String {
  let app_name = &data.app_name;
  let mut counter = data.counter.lock().unwrap();
  *counter += 1;
  format!("Hello {} times {}!", app_name, counter)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let main_data = web::Data::new(AppState {
    app_name: String::from("Rust API"),
    counter: Mutex::new(0),
  });

  let api_data = web::Data::new(AppState {
    app_name: String::from("Rust Main"),
    counter: Mutex::new(0),
  });

  HttpServer::new(move || {
    App::new()
      .service(web::scope("/api")
        .app_data(api_data.clone()).configure(api_configuration))
      .service(web::scope("/main")
        .app_data(main_data.clone()).configure(main_configuration))
  })
  .bind("0.0.0.0:3000")?
    .run()
    .await
}
