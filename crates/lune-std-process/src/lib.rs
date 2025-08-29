#![allow(clippy::cargo_common_metadata)]

use std::{
    env::consts::{ARCH, OS},
    path::MAIN_SEPARATOR,
    process::Stdio,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use mlua::prelude::*;
use mlua_luau_scheduler::Functions;
use signal_hook::consts::signal::*;
use signal_hook::flag;

use lune_utils::{
    TableBuilder,
    path::get_current_dir,
    process::{ProcessArgs, ProcessEnv},
};

mod create;
mod exec;
mod options;

use self::options::ProcessSpawnOptions;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

/**
    Returns a string containing type definitions for the `process` standard library.
*/
#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

/**
    Creates the `process` standard library module.

    # Errors

    Errors when out of memory.
*/
#[allow(clippy::missing_panics_doc)]
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    let mut cwd_str = get_current_dir()
        .to_str()
        .expect("cwd should be valid UTF-8")
        .to_string();
    if !cwd_str.ends_with(MAIN_SEPARATOR) {
        cwd_str.push(MAIN_SEPARATOR);
    }

    // Create constants for OS & processor architecture
    let os = lua.create_string(OS.to_lowercase())?;
    let arch = lua.create_string(ARCH.to_lowercase())?;
    let endianness = lua.create_string(if cfg!(target_endian = "big") {
        "big"
    } else {
        "little"
    })?;

    // Extract stored userdatas for args + env, the runtime struct
    // should always contain and then provide through lua app data
    let process_args = lua
        .app_data_ref::<ProcessArgs>()
        .ok_or_else(|| LuaError::runtime("Missing process args in Lua app data"))?
        .into_plain_lua_table(lua.clone())?;
    let process_env = lua
        .app_data_ref::<ProcessEnv>()
        .ok_or_else(|| LuaError::runtime("Missing process env in Lua app data"))?
        .into_plain_lua_table(lua.clone())?;

    process_args.set_readonly(true);

    // Create our process exit function, the scheduler crate provides this
    let fns = Functions::new(lua.clone())?;
    let process_exit = fns.exit;

    // Create the full process table
    TableBuilder::new(lua.clone())?
        .with_value("os", os)?
        .with_value("arch", arch)?
        .with_value("endianness", endianness)?
        .with_value("args", process_args)?
        .with_value("cwd", cwd_str)?
        .with_value("env", process_env)?
        .with_value("exit", process_exit)?
        .with_async_function("exec", process_exec)?
        .with_function("create", process_create)?
        .with_value("pollSignals", create_process_poll_signals(&lua))?
        .build_readonly()
}

pub fn create_process_poll_signals(lua: &Lua) -> LuaFunction {
    let got_sigint = Arc::new(AtomicBool::new(false));
    let got_sigterm = Arc::new(AtomicBool::new(false));

    // Register OS signal hooks
    flag::register_conditional_default(SIGINT, Arc::clone(&got_sigint))
        .expect("failed to register SIGINT hook");
    flag::register_conditional_default(SIGTERM, Arc::clone(&got_sigterm))
        .expect("failed to register SIGTERM hook");

    lua.create_function(move |_, callback: LuaFunction| {
        if got_sigint.swap(false, Ordering::SeqCst) {
            return callback.call::<()>(SIGINT).into_lua_err();
        }
        if got_sigterm.swap(false, Ordering::SeqCst) {
            return callback.call::<()>(SIGTERM).into_lua_err();
        }
        Ok(())
    })
    .expect("failed to create process.pollSignals function")
}

async fn process_exec(
    lua: Lua,
    (program, args, mut options): (String, ProcessArgs, ProcessSpawnOptions),
) -> LuaResult<LuaTable> {
    let stdin = options.stdio.stdin.take();
    let stdout = options.stdio.stdout;
    let stderr = options.stdio.stderr;

    let stdin_stdio = if stdin.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    };

    let child = options
        .into_command(program, args)
        .stdin(stdin_stdio)
        .stdout(stdout.as_stdio())
        .stderr(stderr.as_stdio())
        .spawn()?;

    exec::exec(lua, child, stdin, stdout, stderr).await
}

fn process_create(
    lua: &Lua,
    (program, args, mut options): (String, ProcessArgs, ProcessSpawnOptions),
) -> LuaResult<LuaValue> {
    let stdin = options.stdio.stdin.take();
    let stdout = options.stdio.stdout;
    let stderr = options.stdio.stderr;

    let stdin_stdio = if stdin.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    };

    let child = options
        .into_command(program, args)
        .stdin(stdin_stdio)
        .stdout(stdout.as_stdio())
        .stderr(stderr.as_stdio())
        .spawn()?;

    create::Child::new(lua, child).into_lua(lua)
}
