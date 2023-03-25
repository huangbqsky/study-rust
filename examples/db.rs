#![allow(dead_code, deprecated)]

use std::collections::HashMap;
use rusqlite::{Connection, Result, NO_PARAMS};

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}
fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    conn.execute(
     "CREATE TABLE IF NOT EXISTS cats_colors (
                   id integer primary key autoincrement, 
                   name text NOT NULL UNIQUE)",
        (),
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cats (
            id integer primary key,
            name text NOT NULL,
            color_id integer Not NULL References cat_colors(id))",
         (),
    )?;
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger","Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in cat_colors {
        conn.execute("INSERT INTO cats_color(name) VALUES (?1)", &[&color.to_string()])?;
        // 获取最近插入数据行的 id
        let last_id: String = conn.last_insert_rowid().to_string();
        for cat in catnames {
            conn.execute("INSERT INTO cats (name, color_id) values (?1, ?2)", 
                  &[&cat.to_string(), &last_id])?;
        }
    }


    let mut stmt = conn.prepare("SELECT c.name, cc.name FROM cats c inner Join cat_colors cc on cc.id = c.color_id;")?;
    let cats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;
    for cat in cats {
        println!("Found cat: {:?}", cat);
    }
    Ok(())  
}

fn successfully_tx(conn: &mut Connection) -> Result<()>{
    let tx = conn.transaction()?;
    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (color) values (?1)", &[&"blue"])?;
    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.commit()
}