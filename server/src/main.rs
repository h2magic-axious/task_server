#[macro_use]
extern crate dotenv_codegen;
// extern crate db;

use sqlx::postgres::PgPoolOptions;
use db::models::Task;


#[tokio::main]
async fn main(){
    println!("Hello, world!");
}
