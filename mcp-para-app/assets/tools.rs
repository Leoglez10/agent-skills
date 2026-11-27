//! Tool specs and dispatch, shape taken from Finanzas (crates/finanzas-mcp/src/tools.rs).
use app_core::store::Store;
use serde_json::{json, Value};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("{0}")]
pub struct ToolError(pub String);

#[derive(Debug)]
pub struct ToolSpec { pub name: &'static str, pub description: &'static str, pub input_schema: Value, pub read_only: bool, pub destructive: bool }

fn spec(name: &'static str, description: &'static str, properties: Value, required: &[&str], read_only: bool, destructive: bool) -> ToolSpec {
    let mut properties = properties;
    // Every tool takes the optional profile, so agents can work on any profile the app created.
    properties.as_object_mut().expect("tool properties are an object")
        .insert("perfil".into(), json!({"type":"string","description":"Profile whose data to use: personal (default) or one created in the app"}));
    ToolSpec { name, description, input_schema: json!({"type":"object","properties":properties,"required":required}), read_only, destructive }
}

pub fn specs() -> Vec<ToolSpec> {
    let date = json!({"type":"string","description":"YYYY-MM-DD"});
    vec![
        spec("list_items", "List all items with id, name and status.", json!({}), &[], true, false),
        spec("add_item", "Add an item. Returns the item as list_items shows it.", json!({"name":{"type":"string"},"date":date}), &["name"], false, false),
        spec("update_item", "Change fields of an item; only the fields given change, at least one is required.", json!({"id":{"type":"string"},"name":{"type":"string"}}), &["id"], false, false),
        spec("delete_item", "Permanently delete an item by id. Returns the deleted item.", json!({"id":{"type":"string","description":"Item id (see list_items)"}}), &["id"], false, true),
    ]
}

pub fn profile(args: &Value) -> Result<String, ToolError> {
    Ok(args.get("perfil").and_then(Value::as_str).unwrap_or("personal").to_owned())
}

pub fn call(store: &mut Store, name: &str, args: &Value) -> Result<Value, ToolError> {
    let text = |key: &str| args.get(key).and_then(Value::as_str).ok_or_else(|| ToolError(format!("{key}: required")));
    match name {
        "list_items" => { let _ = store; todo!("read from the core crate and map to json") }
        "add_item" => { let _ = text("name")?; todo!() }
        "update_item" => { let _ = text("id")?; todo!() }
        "delete_item" => { let _ = text("id")?; todo!() }
        _ => Err(ToolError(format!("unknown tool {name}"))),
    }
}
