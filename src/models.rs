//extern crate chrono;

use rustorm::query::Query;
use rustorm::dao::{Dao, IsDao, ToValue};
use rustorm::table::{IsTable, Table, Column};
use rustorm::database::Database;

pub trait Entity : IsTable + IsDao {
    fn id(&self) -> &ToValue;
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Photo {
    pub id: i32,
    pub path: String,
    pub grade: Option<i16>,
    pub rotation: i16
}

impl Entity for Photo {
    fn id(&self) -> &ToValue {
        &self.id
    }
}
impl IsDao for Photo {
    fn from_dao(dao:&Dao) -> Self {
        Photo {
            id: dao.get("id"),
            path: dao.get("path"),
            grade: dao.get_opt("grade"),
            rotation: dao.get("rotation")
        }
    }
    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("id", &self.id);
        dao.set("path", &self.path);
        if let Some(grade) = self.grade { // NOTE dao.set_opt would be nice?
            dao.set("grade", &grade);
        } else {
            dao.set_null("grade");
        }
        dao.set("rotation", &self.rotation);
        dao
    }
}
impl IsTable for Photo {
    fn table() -> Table {
        table("photo", vec![
            Column {
                name: "id".to_string(),
                data_type: "i32".to_string(),
                db_data_type: "serial".to_string(),
                is_primary: true,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "path".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "grade".to_string(),
                data_type: "i16".to_string(),
                db_data_type: "smallint".to_string(),
                is_primary: false,
                is_unique: false,
                default: None,
                comment: None,
                not_null: false,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "rotation".to_string(),
                data_type: "i16".to_string(),
                db_data_type: "smallint".to_string(),
                is_primary: false,
                is_unique: false,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            }
            ])
    }
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub slug: String,
}

impl Entity for Tag {
    fn id(&self) -> &ToValue {
        &self.id
    }
}
impl IsDao for Tag {
    fn from_dao(dao: &Dao) -> Self {
        Tag {
            id: dao.get("id"),
            tag: dao.get("tag"),
            slug: dao.get("slug"),
        }
    }
    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("id", &self.id);
        dao.set("tag", &self.tag);
        dao.set("slug", &self.slug);
        dao
    }
}

impl IsTable for Tag {
    fn table() -> Table {
        table("tag", vec![
            Column {
                name: "id".to_string(),
                data_type: "i32".to_string(),
                db_data_type: "serial".to_string(),
                is_primary: true,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "tag".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "slug".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            }
            ])
    }
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl Entity for Person {
    fn id(&self) -> &ToValue {
        &self.id
    }
}
impl IsDao for Person {
    fn from_dao(dao: &Dao) -> Self {
        Person {
            id: dao.get("id"),
            name: dao.get("name"),
            slug: dao.get("slug"),
        }
    }
    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("id", &self.id);
        dao.set("name", &self.name);
        dao.set("slug", &self.slug);
        dao
    }
}

impl IsTable for Person {
    fn table() -> Table {
        table("person", vec![
            Column {
                name: "id".to_string(),
                data_type: "i32".to_string(),
                db_data_type: "serial".to_string(),
                is_primary: true,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "name".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "slug".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            }
            ])
    }
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Place {
    pub id: i32,
    pub place: String,
    pub slug: String,
}

impl Entity for Place {
    fn id(&self) -> &ToValue {
        &self.id
    }
}
impl IsDao for Place {
    fn from_dao(dao: &Dao) -> Self {
        Place {
            id: dao.get("id"),
            place: dao.get("place"),
            slug: dao.get("slug"),
        }
    }
    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("id", &self.id);
        dao.set("place", &self.place);
        dao.set("slug", &self.slug);
        dao
    }
}

impl IsTable for Place {
    fn table() -> Table {
        table("place", vec![
            Column {
                name: "id".to_string(),
                data_type: "i32".to_string(),
                db_data_type: "serial".to_string(),
                is_primary: true,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "place".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            },
            Column {
                name: "slug".to_string(),
                data_type: "String".to_string(),
                db_data_type: "varchar(100)".to_string(),
                is_primary: false,
                is_unique: true,
                default: None,
                comment: None,
                not_null: true,
                foreign: None,
                is_inherited: false
            }
            ])
    }
}

fn table(name: &str, columns: Vec<Column>) -> Table {
    Table {
        schema: "public".to_owned(),
        name: name.to_owned(),
        parent_table: None,
        sub_table: vec![],
        comment: None,
        columns: columns,
        is_view: false
    }
}


pub fn query_for<T: IsTable>() -> Query {
    let mut q = Query::select();
    q.from(&T::table());
    q
}

pub fn get_or_create<'a, T: IsTable + IsDao>
    (db: &Database, key: &str, val: &ToValue, defaults: &[(&str, &ToValue)]) -> T
{
    if let Ok(result) = query_for::<T>().filter_eq(key, val).collect_one(db) {
        result
    } else {
        let table = T::table();
        let mut q = Query::insert();
        q.into_(&table);
        q.set(key, val);
        for p in defaults {
            let &(key, f) = p;
            q.set(key, f);
        }
        q.returns(table.columns.iter().map(|c| &*c.name).collect())
            .collect_one(db).unwrap()
    }
}

