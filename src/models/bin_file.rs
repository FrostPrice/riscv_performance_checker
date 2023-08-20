use actix_web::web::Bytes;
use diesel::{Insertable, QueryResult, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::{config::db::Connection, schema::bin_files::dsl::*};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::bin_files)]
pub struct BinFile {
    id: String,
    // architecture: String,
    file: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::bin_files)]
pub struct BinFileDTO {
    id: String,
    // architecture: String,
    file: String,
}

impl BinFile {
    pub fn insert(i: String, new_file: Bytes, conn: &mut Connection) -> QueryResult<usize> {
        let text_file = String::from_utf8(new_file.to_vec()).unwrap();

        let bin_file_dto = BinFileDTO {
            id: i,
            file: text_file,
        };

        diesel::insert_into(bin_files)
            .values(bin_file_dto)
            .execute(conn)
    }
}
