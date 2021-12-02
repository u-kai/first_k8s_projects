use std::env::{self, VarError};
pub fn create_mysql_endpoint() -> Result<String, VarError> {
    let mysql_password = env::var("MYSQL_PASSWORD")?;
    let mysql_database = env::var("MYSQL_DATABASE")?;
    let mysql_host = env::var("MYSQL_HOST")?;
    let mysql_port = env::var("MYSQL_port")?;
    let mysql_endpoint = format!(
        "mysql://root:{}@{}:{}/{}",
        mysql_password, mysql_host, mysql_port, mysql_database
    );
    Ok(mysql_endpoint)
}
