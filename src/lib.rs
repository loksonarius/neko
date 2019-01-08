#[macro_use]
extern crate failure;
extern crate glob;
#[macro_use]
extern crate log;
extern crate loggerv;
extern crate serde_json;
extern crate tera;

use failure::Error;
use glob::glob;
use serde_json::Value;
use std::{fs::File, path::Path};
use tera::Tera;

pub fn parse_data_file<P: AsRef<Path>>(path: &P) -> Result<Value, Error> {
    let file = File::open(path)?;
    let data: Value = serde_json::from_reader(file)?;
    Ok(data)
}

pub fn parse_data_glob(data_glob: &str) -> Result<Value, Error> {
    let mut data = Value::Null;
    for file in glob(data_glob).unwrap() {
        match file {
            Ok(path) => {
                let read_data = parse_data_file(&path)?;
                data = merge_data(&data, &read_data).unwrap()
            },
            Err(e) => {
                warn!("Data path matched but was unreadable -- got error: {}", e);
            },
        }
    }
    Ok(data)
}

fn merge_data(bottom: &Value, top: &Value) -> Option<Value> {
    match (bottom, top) {
        (Value::Object(ref b), Value::Object(ref t)) => {
            let mut result = b.clone();
            for (k, v) in t {
                result.insert(k.clone(), merge_data(b.get(k).unwrap_or(&Value::Null), v)?);
            }
            Some(result.into())
        }
        (Value::Array(ref b), Value::Array(ref t)) => {
            let mut result = b.clone();
            for i in t {
                result.push(i.clone());
            }
            Some(result.into())
        }
        (_, h) => {
            Some(h.clone())
        }
    }
}

pub fn compile_templates(path: &str) -> Result<Tera, Error> {
    match Tera::new(path) {
        Ok(t) => Ok(t),
        Err(e) => {
            error!("Failed to compile templates under '{}' -- got error {}",
                   path, e);
            Err(format_err!("Failed to compile templates"))
        }
    }
}

pub fn render_template(templates: Tera, data: Value, name: &str) -> Result<String, Error> {
    match templates.render(name, &data) {
        Ok(o) => Ok(o),
        Err(e) => {
            error!("Failed to render template '{}' -- got error {}",
                   name, e);
            Err(format_err!("Failed to render template"))
        }
    }
}
