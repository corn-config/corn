use crate::Value;
use mlua::prelude::*;

impl<'lua> IntoLua<'lua> for Value<'lua> {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.to_value(&self)
    }
}

fn lua_parse(lua: &Lua, config: String) -> LuaResult<LuaValue> {
    let res = crate::parse(&config);
    match res {
        Ok(v) => Ok(lua.to_value(&v)?),
        Err(e) => Err(LuaError::RuntimeError(e.to_string())),
    }
}

#[mlua::lua_module]
fn libcorn(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let parse = lua.create_function(lua_parse)?;
    exports.set("parse", parse)?;
    Ok(exports)
}
