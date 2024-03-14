use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub pizza_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Pizza {
        Pizza {
            uuid,
            pizza_name
        }
    }
}
