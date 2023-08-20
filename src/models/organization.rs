use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Insertable, QueryResult, Queryable, RunQueryDsl, SqliteConnection,
};
use serde::{Deserialize, Serialize};

use crate::{config::db::Connection, schema::organizations::dsl::*};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Organization {
    pub id: String,
    pub clock: i32,
    pub cpi_instruction_r: i32,
    pub cpi_instruction_i: i32,
    pub cpi_instruction_l: i32,
    pub cpi_instruction_s: i32,
    pub cpi_instruction_b: i32,
    pub cpi_instruction_u: i32,
    pub cpi_instruction_j: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::organizations)]
pub struct OrganizationDTO {
    pub id: String,
    pub clock: i32,
    pub cpi_instruction_r: i32,
    pub cpi_instruction_i: i32,
    pub cpi_instruction_l: i32,
    pub cpi_instruction_s: i32,
    pub cpi_instruction_b: i32,
    pub cpi_instruction_u: i32,
    pub cpi_instruction_j: i32,
}

impl Organization {
    pub fn insert(new_organization: OrganizationDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(organizations)
            .values(&new_organization)
            .execute(conn)
    }
}
