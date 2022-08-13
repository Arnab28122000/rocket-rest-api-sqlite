#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rusqlite::Connection;

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: i64,
    title: String,
    done: String,
}

impl TodoItem {
    fn new(title: &str) -> TodoItem {
        TodoItem {
            id: 0,
            title: title.to_string(),
            done: "false".to_string(),
        }
    }
}



#[derive(Serialize)]
struct StatusMessage{
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/todo", format = "json" , data = "<item>")]
fn add_todo_item(item: Json<TodoItem>) -> Result<Json<StatusMessage>, String> {
    println!("{:?}", item);
    let todo_item = TodoItem::new(&item.title);
    println!("Title: {}", &item.title.to_string());
    

    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let mut statement = match db_connection.prepare("insert into todo_list (id, title, done) values (null, $1, $2)") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    

    let results = statement.execute(&[&todo_item.title, &todo_item.done.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage { 
            message: format!("{} rows inserted", rows_affected)
         })),
        Err(_) => Err(String::from("Failed to insert todo item"))
    }
}

#[delete("/todo/<id>")]
fn remove(id: i64) -> Result<Json<StatusMessage>, String> {
   
    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let mut statement = match db_connection.prepare("delete from todo_list where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    

    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage { 
            message: format!("{} item deleted ", rows_affected)
         })),
        Err(_) => Err(String::from("Failed to delete todo item"))
    }
}

#[patch("/todo/<id>")]
fn update(id: i64) -> Result<Json<StatusMessage>, String> {
   
    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let mut statement = match db_connection.prepare("update todo_list set done = 'complete' where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    

    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage { 
            message: format!("{} item update ", rows_affected)
         })),
        Err(_) => Err(String::from("Failed to update todo item"))
    }
}


#[get("/todo")]
fn fetch_all_todo_items() -> Result<Json<TodoList>, String> {
    let db_connection = match Connection::open("db.sqlite") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let mut statement = match db_connection.prepare("select id, title, done from todo_list") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    

    let results = statement.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            title: row.get(1)?,
            done: row.get(2)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<TodoItem>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(TodoList { items })),
                Err(err) => Err(String::from(err.to_string())),
            }
        }
        Err(_) => Err(String::from("Error while fetching todo items"))
    }
}

fn main() {
    {
    let db_connection = Connection::open("db.sqlite").unwrap();

    // db_connection.execute("drop table if exists todo_list;", []).expect("Error creating table");
    

    db_connection.execute("create table if not exists todo_list (
        id integer primary key,
        title varchar(64) not null,
        done varchar(64) not null
    );", []).expect("Error creating table");
    }
   
    rocket::ignite()
        .mount("/", routes![index, fetch_all_todo_items, add_todo_item, remove, update]).launch();
       
}