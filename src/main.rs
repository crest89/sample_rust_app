use rust_webapp::{init_db, users_api};

#[tokio::main]
async fn main() {
    let database = init_db();

    warp::serve(users_api(database))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
