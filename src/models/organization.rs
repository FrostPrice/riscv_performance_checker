use diesel::{Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::{config::db::Connection, schema::organizations::dsl::*};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::organizations)]
pub struct Organization {
    pub id: String,
    pub clock: f32,
    pub cpi_instruction_r: f32,
    pub cpi_instruction_i: f32,
    pub cpi_instruction_l: f32,
    pub cpi_instruction_s: f32,
    pub cpi_instruction_b: f32,
    pub cpi_instruction_u: f32,
    pub cpi_instruction_j: f32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::organizations)]
pub struct OrganizationDTO {
    pub id: String,
    pub clock: f32,
    pub cpi_instruction_r: f32,
    pub cpi_instruction_i: f32,
    pub cpi_instruction_l: f32,
    pub cpi_instruction_s: f32,
    pub cpi_instruction_b: f32,
    pub cpi_instruction_u: f32,
    pub cpi_instruction_j: f32,
}

impl Organization {
    pub fn find_by_id(i: String, conn: &mut Connection) -> QueryResult<Organization> {
        organizations
            .select((
                id,
                clock,
                cpi_instruction_r,
                cpi_instruction_i,
                cpi_instruction_l,
                cpi_instruction_s,
                cpi_instruction_b,
                cpi_instruction_u,
                cpi_instruction_j,
            ))
            .find(i)
            .get_result::<Organization>(conn)
    }

    pub fn insert(new_organization: OrganizationDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(organizations)
            .values(&new_organization)
            .execute(conn)
    }
}
