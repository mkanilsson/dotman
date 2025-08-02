use std::{fs, path::Path};

use crate::{
    errors::{DotManResult, Error},
    print::Printer,
    utils::{expand, from_cwd, serde_json_value_to_mlua_value},
};
use mlua::{Function, IntoLua, Lua, LuaSerdeExt, Value};

pub struct Script {
    post_install: Option<mlua::Function>,
    post_update: Option<mlua::Function>,

    cwd: String,
    lua: mlua::Lua,
}

impl Script {
    pub fn load(dir: &str, pp: &Printer) -> DotManResult<Self> {
        let path = Path::new(dir).join(".dotman.lua");
        if !path.exists() {
            return Err(Error::MissingScript);
        }

        let lua = Lua::new();
        let string = fs::read_to_string(path)?;
        let chunk = lua.load(string);
        let module = chunk.eval::<Value>()?;

        match module {
            Value::Table(t) => {
                let post_install = t.get::<Function>("post_install")?;
                let post_update = t.get::<Function>("post_update")?;

                let script = Self {
                    post_install: Some(post_install),
                    post_update: Some(post_update),
                    cwd: dir.to_string(),
                    lua,
                };
                script.populate_globals()?;
                return Ok(script);
            }
            _ => {
                pp.error("Expected object returned from .dotman.lua to return a table");
                return Err(Error::MissingScript);
            }
        }
    }

    fn populate_globals(&self) -> DotManResult<()> {
        let globals = self.lua.globals();
        let cwd = self.cwd.to_string();

        let symlink_fn = self.lua.create_function(
            move |lua: &Lua, (destination, target): (String, String)| {
                let destination = match expand(&cwd, &destination) {
                    Ok(a) => a,
                    Err(_) => return false.into_lua(lua),
                };
                let target = match expand(&cwd, &target) {
                    Ok(a) => a,
                    Err(_) => return false.into_lua(lua),
                };

                println!("symlink {destination} -> {target}");

                if let Err(e) = std::os::unix::fs::symlink(destination, target) {
                    Error::from(e).print_warning();
                    false
                } else {
                    true
                }
                .into_lua(lua)
            },
        )?;

        globals.set("symlink", symlink_fn)?;

        self.populate_fs()?;
        self.populate_json()?;

        Ok(())
    }

    fn populate_fs(&self) -> DotManResult<()> {
        let fs_table = self.lua.create_table()?;
        let cwd = self.cwd.to_string();

        let read_to_string_fn = self.lua.create_function(move |lua: &Lua, path: String| {
            let path = match expand(&cwd, &path) {
                Ok(a) => a,
                Err(_) => return Ok(Value::Nil),
            };

            println!("read_to_string {path}");

            match fs::read_to_string(path) {
                Ok(data) => data.into_lua(lua),
                Err(e) => {
                    Error::from(e).print_warning();
                    Ok(Value::Nil)
                }
            }
        })?;

        let cwd = self.cwd.to_string();
        let expand_fn = self.lua.create_function(move |lua: &Lua, path: String| {
            println!("expand {path}");

            let path = match expand(&cwd, &path) {
                Ok(a) => a,
                Err(_) => return Ok(Value::Nil),
            };

            return path.into_lua(lua);
        })?;

        let cwd = self.cwd.to_string();
        let from_cwd_fn = self.lua.create_function(move |lua: &Lua, path: String| {
            println!("from_cwd {path}");

            let path = match from_cwd(&cwd, &path) {
                Ok(a) => a,
                Err(_) => return Ok(Value::Nil),
            };

            return path.into_lua(lua);
        })?;

        fs_table.set("read_to_string", read_to_string_fn)?;
        fs_table.set("from_cwd", from_cwd_fn)?;
        fs_table.set("expand", expand_fn)?;

        self.lua.globals().set("fs", fs_table)?;

        Ok(())
    }

    fn populate_json(&self) -> DotManResult<()> {
        let json_table = self.lua.create_table()?;

        let parse_fn = self.lua.create_function(move |lua: &Lua, json: String| {
            println!("parse {json}");

            let obj = match serde_json::from_str(&json) {
                Ok(obj) => obj,
                Err(e) => {
                    Error::from(e).print_warning();
                    return Ok(Value::Nil);
                }
            };

            match serde_json_value_to_mlua_value(lua, obj) {
                Ok(value) => value.into_lua(lua),
                Err(e) => {
                    Error::from(e).print_warning();
                    return Ok(Value::Nil);
                }
            }
        })?;

        let to_string_fn = self.lua.create_function(move |lua: &Lua, obj: Value| {
            println!("to_string");

            match serde_json::to_string(&obj) {
                Ok(obj) => obj.into_lua(lua),
                Err(e) => {
                    Error::from(e).print_warning();
                    return Ok(Value::Nil);
                }
            }
        })?;

        json_table.set("parse", parse_fn)?;
        json_table.set("to_string", to_string_fn)?;

        self.lua.globals().set("json", json_table)?;

        Ok(())
    }

    pub fn run_postinstall(&self) -> DotManResult<()> {
        if let Some(post_install) = &self.post_install {
            post_install.call::<Value>(())?;
        }

        Ok(())
    }

    pub fn run_postupdate(&self) -> DotManResult<()> {
        if let Some(post_update) = &self.post_update {
            post_update.call::<Value>(())?;
        }

        Ok(())
    }
}
