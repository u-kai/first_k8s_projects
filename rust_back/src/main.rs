use std::collections::HashMap;
use std::env::{self, VarError};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, App, HttpResponse, HttpServer};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use tokio;

#[get("/getjson")]
async fn index() -> HttpResponse {
    let users = get_users().unwrap();
    let data = ReJson { data: users };
    let seri = serde_json::to_string(&data).unwrap();
    println!("{:?}", seri);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(seri)
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct SampleData {
    user_id: i32,
    name: String,
    password: String,
}
#[get("/create")]
async fn create() -> HttpResponse {
    let mysql_password = env::var("MYSQL_ROOT_PASSWORD").unwrap();
    //let mysql_database = "mysql";//env::var("MYSQL_DATABASE").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    let mysql_port = env::var("MYSQL_PORT").unwrap();
    let mysql_endpoint = format!(
        "mysql://root:{}@{}:{}",
        mysql_password, mysql_host, mysql_port
    );
    println!("endpoint {:?}", mysql_endpoint);
    let url = Opts::from_url(&mysql_endpoint).unwrap();
    println!("url {:?}", url);
    let pool = match Pool::new(url) {
        Ok(p) => {
            println!("pool {:?}", p);
            p
        }
        Err(e) => {
            return HttpResponse::Ok().body(format!(
                "<div>pool: error {}</div><div>endpoint: {}</div>",
                e, mysql_endpoint
            ))
        }
    };
    let mut conn = match pool.get_conn() {
        Ok(c) => {
            println!("conn {:?}", c);
            c
        }
        Err(e) => return HttpResponse::Ok().body(format!("<div>conn: error {}</div>", e)),
    };
    let schema_name = "sample_schema";
    let create_schema = format!("create schema if not exists {}", schema_name);
    conn.query_drop(create_schema).unwrap();
    let create_table_sql = format!(
        "create table if not exists {}.sample (user_id int,name varchar(40),password varchar(256))",
        schema_name
    );
    conn.query_drop(create_table_sql).unwrap();

    let insert_datas = vec![
        SampleData {
            user_id: 0,
            name: String::from("kai"),
            password: String::from("kaikai"),
        },
        SampleData {
            user_id: 1,
            name: String::from("mio"),
            password: String::from("miomio"),
        },
        SampleData {
            user_id: 2,
            name: String::from("minami"),
            password: String::from("minamina"),
        },
        SampleData {
            user_id: 3,
            name: String::from("yui"),
            password: String::from("yuiyui"),
        },
    ];

    conn.exec_batch(
        format!(
            r"insert into {}.sample (user_id,name,password) values (:user_id,:name,:password)",
            schema_name
        ),
        insert_datas.iter().map(|s| {
            params! {
                "user_id"=>&s.user_id,
                "name"=>&s.name,
                "password"=>&s.password
            }
        }),
    )
    .unwrap();
    let samples: Vec<SampleData> = conn
        .query_map(
            format!("select user_id,name,password from {}.sample", schema_name),
            |(user_id, name, password)| SampleData {
                user_id,
                name,
                password,
            },
        )
        .unwrap();
    let seri = serde_json::to_string(&samples).unwrap();
    println!("{:?}", samples);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(seri)
}

#[derive(Serialize, Deserialize, Debug)]
struct RustBack2 {
    data: HashMap<String, String>,
}
#[get("/")]
#[tokio::main]
async fn hello() -> HttpResponse {
    let dist = env::var("DIST").unwrap();
    let resp = reqwest::get(dist).await.unwrap().text().await.unwrap();
    let desiri: RustBack2 = serde_json::from_str(&resp).unwrap();
    let data = desiri.data;
    let data: HashMap<_, _> = data
        .iter()
        .map(|(k, v)| {
            let new_key = format!("key:{}", k);
            let new_value = format!("value:{}", v);
            (new_key, new_value)
        })
        .collect();
    let seri = serde_json::to_string(&data).unwrap();
    HttpResponse::Ok()
        .content_type("appliaction/json")
        .body(format!(r#"{}"#, seri))
}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}
#[get("/request")]
#[tokio::main]
async fn request() -> HttpResponse {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", resp);
    let desiri: Todo = serde_json::from_str(&resp).unwrap();
    println!("{:?}", desiri);
    HttpResponse::Ok()
        .content_type("appliaction/json")
        .body(r#"{"data":"hello"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .service(index)
            .service(hello)
            .service(request)
            .service(create)
            .wrap(cors)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct ReJson {
    data: Vec<UserLogin>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct UserLogin {
    user_id: i32,
    name: String,
    password: String,
}

fn get_users() -> Result<Vec<UserLogin>> {
    let mut conn = establish_connection()?;
    let users = conn.query_map(
        "select user_id,name,password from users_login",
        |(user_id, name, password)| UserLogin {
            user_id,
            name,
            password,
        },
    )?;
    Ok(users)
}

fn establish_connection() -> Result<PooledConn> {
    let mysql_password = env::var("MYSQL_ROOT_PASSWORD").unwrap();
    let mysql_database = env::var("MYSQL_DATABASE").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    let mysql_port = env::var("MYSQL_PORT").unwrap();
    let mysql_endpoint = format!(
        "mysql://root:{}@{}:{}/{}",
        mysql_password, mysql_host, mysql_port, mysql_database
    );
    let url = Opts::from_url(&mysql_endpoint).unwrap();
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}
