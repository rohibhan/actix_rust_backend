use actix_web::{
    get, patch, post, web::Data, web::Json, web::Path, App, HttpServer, Responder,
};
mod models;
use crate::models::{BuyPizzaRequest, Pizza, UpdatePizzaURL};
mod db;
mod error;

use crate::db::{Database,pizza_data_trait::PizzaDataTrait};
use crate::error::PizzaError;
use uuid;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizza(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = Database::get_all_pizza(&db).await;
    match pizzas {
        Some(all_pizzas) => Ok(Json(all_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

#[post("/buypizza")]
async fn buy_pizza(
    body: Json<BuyPizzaRequest>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = Database::add_pizza(&db,Pizza::new(String::from(new_uuid), pizza_name))
                .await;

            match new_pizza {
                Some(pizza) => Ok(Json(pizza)),
                None => Err(PizzaError::PizzaCreationFailed),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreationFailed),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(
    update_pizza_url: Path<UpdatePizzaURL>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = Database::update_pizza(&db,uuid).await;
    match update_result {
        Some(pizza) => Ok(Json(pizza)),
        None => Err(PizzaError::NoSuchPizzaFound),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizza)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
