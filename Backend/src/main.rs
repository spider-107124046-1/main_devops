//To actually use them
mod services { 
    //Adding pub makes it public and we can use the functions in the file
    pub mod db; 
    pub mod endpoints;
}
mod models {
    pub mod models;
}
//Using root level file
pub mod schema;
//By keeping all these files in main, [Intellisense/Rust sees them]

use actix_web::{web, error,App,middleware, HttpResponse, HttpServer, Responder};

//For logging
use env_logger::Env;
use crate::services::endpoints::{
    create_new_user,get_all_present_user,
    get_some_user,update_particular_user,
    delete_particular_user,login_user};
// use crate::services::endpoints::get_user;
use crate::services::db::get_connection_pool;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}


//Making a conneciton pool
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    

    let pool = get_connection_pool();
    println!("Connection to DB Established !\n");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {

        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(json_config)
            .app_data(web::Data::new(pool.clone()))
            .service(create_new_user)
            .service(get_all_present_user)
            .service(get_some_user)
            .service(update_particular_user)
            .service(delete_particular_user)
            .service(login_user)
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}