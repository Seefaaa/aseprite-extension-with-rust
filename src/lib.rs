mod http;
mod jobs;
mod luasrc;

use mlua::prelude::*;

#[mlua::lua_module]
fn libtest(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("hello", lua.create_function(|_, ()| Ok("Hello, World!"))?)?;
    exports.set(
        "http_get",
        lua.create_function(|_, url: String| {
            Ok(jobs::start(move || {
                http::get(&url).unwrap_or_else(|e| e.to_string())
            }))
        })?,
    )?;

    exports.set(
        "check_job",
        lua.create_function(|_, id: String| Ok(jobs::check(&id)))?,
    )?;

    // lua.globals().set("libtest", exports)?;

    Ok(exports)
}
