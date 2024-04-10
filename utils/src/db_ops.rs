use std::fs;

use sqlx::{Executor, MySql, MySqlPool, Pool};

use crate::environ::Environ;

#[allow(async_fn_in_trait)]
pub trait CanManageDbConnections {
    async fn setup_durable_database(&self, connection_string: String) -> Result<MySqlPool, sqlx::Error>;
}

pub struct DbOps;

impl CanManageDbConnections for DbOps {
    async fn setup_durable_database(&self, connection_string: String) -> Result<MySqlPool, sqlx::Error> {
        let pool = MySqlPool::connect(connection_string.as_str()).await;

        match pool {
            Ok(p) => Ok(p),
            Err(e) => Err(e),
        }
    }
}

// pub async fn setup_durable_database(connection_string: String) -> Result<MySqlPool, sqlx::Error> {
//     let pool = MySqlPool::connect(connection_string.as_str()).await;

//     match pool {
//         Ok(p) => Ok(p),
//         Err(e) => Err(e),
//     }
// }

pub async fn setup_test_database(db_name: &str) -> MySqlPool {
    let env = Environ::default();
    let database_url = &env.db_connection_string;

    let full_url = format!("{}/{}", database_url, db_name);
    let pool = MySqlPool::connect(database_url).await.expect("Failed to connect to database");

    // Create a new database
    pool.execute(format!("CREATE DATABASE `{}`;", db_name).as_str()).await.expect("Failed to create database");

    // Connect to the new database
    let pool = MySqlPool::connect(&full_url).await.expect("Failed to connect to new database");

    // Run schema.sql file
    let schema = fs::read_to_string("../database/original_schema.sql").expect("Failed to read schema.sql file");
    pool.execute(schema.as_str()).await.expect("Failed to execute schema.sql");

    pool
}

pub async fn teardown_test_database(pool: &Pool<MySql>, db_name: &str) -> Result<(), sqlx::Error> {
    // switch to mysql database then execute the drop database command from there
    match pool.execute("use mysql;").await {
        Ok(_) => {
            println!("switched to mysql");
            let drop_command = format!("DROP DATABASE `{}`;", db_name);
            match pool.execute(drop_command.as_str()).await {
                Ok(_) => {
                    println!("Database {} dropped successfully.", db_name);
                    pool.close().await;
                    assert!(pool.is_closed());
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to drop the database {}: {}", db_name, e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to switch to mysql");
            Err(e)
        }
    }
}
