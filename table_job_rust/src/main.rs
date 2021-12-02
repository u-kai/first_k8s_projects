use mysql::prelude::*;
use mysql::*;
use std::env;
fn main() {
    let mysql_password = env::var("MYSQL_ROOT_PASSWORD").unwrap();
    //let mysql_database = "mysql";//env::var("MYSQL_DATABASE").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    let mysql_port = env::var("MYSQL_PORT").unwrap();
    let mysql_endpoint = format!(
        "mysql://root:{}@{}:{}",
        mysql_password, mysql_host, mysql_port
    );
    let url = Opts::from_url(&mysql_endpoint).unwrap();
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
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
    println!("{:?}", samples)
}

#[derive(Debug, PartialEq, Eq)]
struct SampleData {
    user_id: i32,
    name: String,
    password: String,
}
