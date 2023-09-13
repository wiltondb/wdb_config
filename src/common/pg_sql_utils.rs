
use std::thread;
use std::time::Duration;
use std::time::Instant;

use chrono::DateTime;
use chrono::Local;
use postgres::Client;
use postgres::Row;
use postgres_types::FromSqlOwned;
use postgres_types::ToSql;
use uuid::Uuid;

use super::*;

pub fn quote_parameter(param: &str) -> String {
    let uid = format!("_{}", Uuid::new_v4()).replace("-", "_");
    format!("${}${}${}$", uid, param, uid)
}

pub fn query_single_row(client: &mut Client, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, PgAccessError> {
    let mut vec = client.query(sql, params)?;
    if 1 != vec.len() {
        return Err(PgAccessError::from_string(format!(
            "Invalid number of records returned, expected: [1], actual: [{}], query: [{}]", vec.len(), sql)))
    }
    let row = vec.pop().ok_or("Invalid vector size")?;
    Ok(row)
}

pub fn query_single_value<T: FromSqlOwned>(client: &mut Client, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<T, PgAccessError> {
    let row = query_single_row(client, sql, params)?;
    let val = row.try_get(0)?;
    Ok(val)
}

pub fn reload_settings_sync(client: &mut Client, timeout_millis: usize) -> Result<(), PgAccessError> {
    let load_moment_before: DateTime<Local> = query_single_value(client, "select pg_conf_load_time()", &[])?;
    client.execute("select pg_reload_conf()", &[])?;
    let start = Instant::now();
    while timeout_millis as i64 - start.elapsed().as_millis() as i64 > 0 {
        let load_moment: DateTime<Local> = query_single_value(client, "select pg_conf_load_time()", &[])?;
        if load_moment_before != load_moment {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}