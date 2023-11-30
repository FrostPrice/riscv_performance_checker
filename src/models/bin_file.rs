use actix_web::web::Bytes;
use diesel::{
    Insertable, query_builder::AsChangeset, Queryable, QueryDsl, QueryResult, RunQueryDsl,
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::{config::db::Connection, schema::bin_files::dsl::*};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::bin_files)]
pub struct BinFile {
    pub id: String,
    // architecture: String,
    pub file: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::bin_files)]
pub struct BinFileDTO {
    id: String,
    // architecture: String,
    file: String,
}

impl BinFile {
    pub fn find_by_id(i: String, conn: &mut Connection) -> QueryResult<BinFile> {
        bin_files
            .select((id, file))
            .find(i)
            .get_result::<BinFile>(conn)
    }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<BinFile>> {
        bin_files.select((id, file)).load::<BinFile>(conn)
    }

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

    pub fn update(i: String, new_file: Bytes, conn: &mut Connection) -> QueryResult<usize> {
        let text_file = String::from_utf8(new_file.to_vec()).unwrap();

        let bin_file_dto = BinFileDTO {
            id: i.clone(),
            file: text_file,
        };

        diesel::update(bin_files.find(i))
            .set(bin_file_dto)
            .execute(conn)
    }

    pub fn delete(i: String, conn: &mut Connection) -> QueryResult<usize> {
        let bin_file = BinFile::find_by_id(i.clone(), conn);

        match bin_file {
            Ok(_) => diesel::delete(bin_files.find(i)).execute(conn),
            Err(err) => Err(err),
        }
    }
}
