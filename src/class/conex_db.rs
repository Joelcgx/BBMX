//Librerias
use mysql::*;
use mysql::prelude::Queryable;

//Funcion Iniciar SEsion
pub fn log_in(username: &str, passw: &str) -> Result<(), mysql::Error> {
    let opts = OptsBuilder::new()
        .user(Some("root"))
        .pass(Some(""))
        .db_name(Some("bbmx"));

    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;
    Ok(())
}