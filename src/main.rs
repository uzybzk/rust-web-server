use warp::Filter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

type UsersDb = Arc<Mutex<HashMap<String, User>>>;

#[tokio::main]
async fn main() {
    // In-memory database
    let users_db: UsersDb = Arc::new(Mutex::new(HashMap::new()));
    
    // GET /users - List all users
    let get_users = warp::path("users")
        .and(warp::get())
        .and(with_db(users_db.clone()))
        .and_then(get_users_handler);
    
    // POST /users - Create a new user
    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(users_db.clone()))
        .and_then(create_user_handler);
    
    // GET /users/:id - Get user by ID
    let get_user = warp::path!("users" / String)
        .and(warp::get())
        .and(with_db(users_db.clone()))
        .and_then(get_user_handler);
    
    // DELETE /users/:id - Delete user by ID
    let delete_user = warp::path!("users" / String)
        .and(warp::delete())
        .and(with_db(users_db.clone()))
        .and_then(delete_user_handler);
    
    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "status": "healthy",
                "service": "rust-web-server"
            }))
        });
    
    // Combine all routes
    let routes = get_users
        .or(create_user)
        .or(get_user) 
        .or(delete_user)
        .or(health)
        .with(warp::cors().allow_any_origin());
    
    println!("Starting Rust web server on http://127.0.0.1:3030");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_db(db: UsersDb) -> impl Filter<Extract = (UsersDb,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn get_users_handler(db: UsersDb) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().unwrap();
    let users_list: Vec<User> = users.values().cloned().collect();
    Ok(warp::reply::json(&users_list))
}

async fn create_user_handler(
    new_user: CreateUserRequest,
    db: UsersDb,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        name: new_user.name,
        email: new_user.email,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    let mut users = db.lock().unwrap();
    users.insert(user_id, user.clone());
    
    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::StatusCode::CREATED,
    ))
}

async fn get_user_handler(id: String, db: UsersDb) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().unwrap();
    match users.get(&id) {
        Some(user) => Ok(warp::reply::json(user)),
        None => Err(warp::reject::not_found()),
    }
}

async fn delete_user_handler(id: String, db: UsersDb) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().unwrap();
    match users.remove(&id) {
        Some(_) => Ok(warp::reply::with_status(
            "User deleted",
            warp::http::StatusCode::NO_CONTENT,
        )),
        None => Err(warp::reject::not_found()),
    }
}