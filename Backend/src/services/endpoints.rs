use actix_web::{Responder,web,Error,post,get,put, HttpResponse, delete};
use crate::models::models::{CreateUser,UpdateUser, Login};
use crate::DbPool;
use crate::services::db::{create_user,get_all_users,get_users,update_user,delete_user,check_login};

#[post("/createUser")]
pub async fn create_new_user(pool: web::Data<DbPool>,mut info : web::Json<CreateUser>) -> Result<impl Responder, Error>{
    let new_user = web::block(move || {
        let conn = &mut pool.get().unwrap();
        create_user(conn,&mut info)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(new_user))
}

#[post("/loginUser")]
pub async fn login_user(pool: web::Data<DbPool>, credentials : web::Json<Login>) -> Result<impl Responder, Error>{
      let login_message = web::block(move || {
        let conn = &mut pool.get().unwrap();
        check_login(conn,&mut credentials.into_inner())
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(login_message))
}

#[get("/getUser")]
pub async fn get_all_present_user(pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let found_users = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_users(conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(found_users))
}

#[get("/getUser/{email}")]
pub async fn get_some_user(email: web:: Path <String>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let details = email.into_inner();
      let found_user = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_users(conn,&details)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(found_user))
}

#[put("/updateUser/{email}")]
pub async fn update_particular_user(email: web:: Path <String>,info : web::Query<UpdateUser>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
  let details = email.into_inner();

      let updated_user = web::block(move || {
        let conn = &mut pool.get().unwrap();
        update_user(conn,&details,&info.into_inner())
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(updated_user))
}

#[delete("/deleteUser/{email}")]
pub async fn delete_particular_user(email: web:: Path <String>,pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
  let details = email.into_inner();
      let results = web::block(move || {
        let conn = &mut pool.get().unwrap();
        delete_user(conn,&details)
      })
      .await?
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(results)
}