
use std::collections::HashMap;

use serde_json::{Value, Error};
use serde::Deserialize;

fn main () -> Result<(), Error> {
    let j = r#"{
        "userid": 103609,
        "verified": true,
        "access_privileges": [
          "user",
          "admin"
        ]
      }"#;
    let parsed: Value = serde_json::from_str(j)?;
    let expected = serde_json::json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });
    assert_eq!(parsed, expected);

    toml_json();
    Ok(())
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}
#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn toml_json() {
    let toml_content = r#"
    [package]
    name = "your_package"
    version = "0.1.0"
    authors = ["You! <you@example.org>"]

    [dependencies]
    serde = "1.0"
    "#;
   let package_info: Config = toml::from_str(toml_content).unwrap();
   println!("{}\n{}\n{:?}\n{:?}", package_info.package.name, package_info.package.version,
         package_info.package.authors, package_info.dependencies["serde"]);
}