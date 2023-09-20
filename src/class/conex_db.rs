//Librerias
use mysql::OptsBuilder;
use std::fmt::Debug;
use mysql::Row;
use mysql::prelude::*;

pub fn verificar_user(username: &str, password: &str) -> Result<bool, mysql::Error> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .user(Some("root"))
        .pass(Some(""))
        .tcp_port(3306)
        .db_name(Some("bbmx"));

    let pool = mysql::Pool::new(opts)?;

    let query = format!(
        "SELECT * FROM usuarios WHERE username = '{}' AND password = '{}'",
        username,
        password
    );
    let result = pool.get_conn()?.query_map(query, |row: Row| -> bool {
        let count: i64 = row.get(0).unwrap();
        count > 0
    })?;

    if let Some(first_result) = result.first() {
        Ok(*first_result)
    } else {
        Ok(false)
    }
   
}
