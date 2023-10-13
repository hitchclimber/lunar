mod models;
use crate::models::models::{MoonDocument, UpdateConfig, MAC};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use log::debug;
use mongodb::{
    bson::{doc, DateTime, Document},
    options::ClientOptions,
    Client, Database,
};
use serde_json::{json, Value};
use std::env;

/// basic health check
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Actix up and running")
}

/// Update 'lastConnected' via `ping` endpoint
#[post("/ping/{id}")]
async fn ping(data: web::Data<Database>, path: web::Path<String>) -> impl Responder {
    let object_id = path.parse::<mongodb::bson::oid::ObjectId>();
    if object_id.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }
    let collection = data.collection::<MoonDocument>("moon");
    let moon = match collection
        .find_one(doc! { "_id": object_id.clone().unwrap() }, None)
        .await
    {
        Ok(moon) => {
            debug!("Found correct moon");
            moon.unwrap()
        }
        Err(_) => return HttpResponse::NotFound().body("Moon not found"),
    };

    let last_time = moon.lastConnected.to_string();
    let _ = collection
        .update_one(
            doc! { "_id": moon._id },
            doc! { "$set": { "lastConnected": DateTime::now() } },
            None,
        )
        .await;

    HttpResponse::Ok().body(last_time)
}

/// Register a new moonBattery by its MAC address
#[post("/register")]
async fn register(data: web::Data<Database>, path: web::Json<MAC>) -> impl Responder {
    let collection = data.collection::<Document>("moon");
    let mac_address = path.macAddress.clone();
    let last_connected = DateTime::now();
    let insert_doc = collection
        .insert_one(
            doc! { "lastConnected": last_connected, "macAddress": mac_address },
            None,
        )
        .await;

    if insert_doc.is_err() {
        debug!("error: {}", insert_doc.err().unwrap());
        return HttpResponse::InternalServerError().body("Error while inserting document");
    };

    HttpResponse::Ok().body(insert_doc.unwrap().inserted_id.to_string())
}

/// Update configuration data for a moonBattery
#[post("/configuration")]
async fn configuration(data: web::Data<Database>, path: web::Json<UpdateConfig>) -> impl Responder {
    let object_id = path._id.parse::<mongodb::bson::oid::ObjectId>();
    if object_id.is_err() {
        return HttpResponse::BadRequest().body("Invalid ID");
    };

    debug!("Deserializing configData worked");

    let collection = data.collection::<MoonDocument>("moon");
    let update_data = &path.configData;
    let config_list = match update_data {
        Value::Object(config_map) => config_map
            .iter()
            .map(|(k, v)| {
                doc! { k: v.to_string() }
            })
            .collect::<Vec<Document>>(),
        _ => return HttpResponse::BadRequest().body("Invalid configData"),
    };

    let _ = collection
        .update_one(
            doc! { "_id": object_id.clone().unwrap()},
            doc! { "$push": { "configData": { "$each": config_list }}},
            None,
        )
        .await;

    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set");
    let client_options = ClientOptions::parse(&database_url).await.unwrap();

    // let it panic, without DB connectino there's no point in running the server
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(mongo_db_name.as_str());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(health)
            .service(ping)
            .service(register)
            .service(configuration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
