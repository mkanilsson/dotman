use std::path::Path;

use mlua::{Lua, Value as LuaValue};
use serde_json::Value as JSONValue;

use crate::errors::{DotManResult, Error};

pub fn expand(cwd: &str, str: &str) -> DotManResult<String> {
    let home_path = match std::env::var("HOME") {
        Ok(path) => Ok(path),
        Err(_) => Err(Error::MissingHomeVariable),
    }?;

    let path = str.replace("$HOME", &home_path).replace("~", &home_path);
    let cwd = cwd.replace("$HOME", &home_path).replace("~", &home_path);

    return from_cwd(&cwd, &path);
}

pub fn from_cwd(cwd: &str, path: &str) -> DotManResult<String> {
    if path.starts_with("/") {
        return Ok(path.to_string());
    } else {
        let base_path = Path::new(cwd);
        return Ok(base_path.join(path).to_string_lossy().to_string());
    }
}

pub fn serde_json_value_to_mlua_value(lua: &Lua, json: JSONValue) -> DotManResult<LuaValue> {
    let lua_value = match json {
        JSONValue::Null => LuaValue::Nil,
        JSONValue::Bool(val) => LuaValue::Boolean(val),
        JSONValue::Array(elements) => {
            let table = lua.create_table()?;

            for element in elements {
                table.push(serde_json_value_to_mlua_value(lua, element)?)?;
            }

            LuaValue::Table(table)
        }
        JSONValue::Number(num) => {
            if num.is_i64() {
                LuaValue::Integer(num.as_i64().unwrap())
            } else if num.is_u64() {
                LuaValue::Integer(num.as_u64().unwrap() as i64)
            } else if num.is_f64() {
                LuaValue::Number(num.as_f64().unwrap())
            } else {
                todo!("Some error message maybe??")
            }
        }
        JSONValue::String(val) => LuaValue::String(lua.create_string(val)?),
        JSONValue::Object(data) => {
            let table = lua.create_table()?;

            for (key, val) in data {
                table.set(key, serde_json_value_to_mlua_value(lua, val)?)?;
            }

            LuaValue::Table(table)
        }
    };

    Ok(lua_value)
}
