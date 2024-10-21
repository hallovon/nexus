use std::sync::Arc;

use nexus_core::model::{LoginRequest, LoginResponse};
use nexus_server::{
    db::{self, users},
    DbPool,
};
use sea_orm::Database;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use warp::{reject::Rejection, reply::Reply, Filter};

const NEXUS_SERVER: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let database_url = "postgres://postgres:root@localhost:5432/postgres";
    let db = Arc::new(Database::connect(database_url).await.unwrap());

    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handle_login);

    warp::serve(login_route).run(([127, 0, 0, 1], 9000)).await;
}

// 登录请求处理函数
async fn handle_login(login: LoginRequest, db: DbPool) -> Result<impl Reply, Rejection> {
    if let Some(user) = users::find_user_by_username(db, &login.username).await {
        if user.password_hash == login.password {
            let token = format!("token-{}", login.username);
            return Ok(warp::reply::json(&LoginResponse {
                message: "Login successful!".to_string(),
                token: Some(token),
                is_success: true,
            }));
        }
    }
    Ok(warp::reply::json(&LoginResponse {
        message: "Invalid credentials.".to_string(),
        token: None,
        is_success: false,
    }))
}

// Warp 路由需要的共享状态过滤器
fn with_db(
    db: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
