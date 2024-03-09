mod models;

use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use crate::models::BuyPizzaRequest;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}



#[post("/buypizza")]
async fn buy_pizzas(body:Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) =>{
            let pizza_name = &body.pizza_name;
            HttpResponse::Ok().body(format!("Pizzas bought is {pizza_name}!"))
        },
        Err(e) =>   HttpResponse::Ok().body("Pizzas name required")
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizzas updated!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizzas)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
