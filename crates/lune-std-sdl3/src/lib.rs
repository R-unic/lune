#![allow(clippy::cargo_common_metadata)]

use mlua::prelude::*;
use sdl3::pixels::Color;

use lune_utils::TableBuilder;

mod context;
use context::init;
use context::{ColorWrapper, FPointWrapper, FRectWrapper, PointWrapper, RectWrapper};
use sdl3::rect::{Point, Rect};
use sdl3::render::{FPoint, FRect};

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

/**
    Returns a string containing type definitions for the `sdl3` extension library.
*/
#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

struct ColorConstructors;
impl LuaUserData for ColorConstructors {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("rgb", rgb);
        methods.add_function("rgba", rgba);
    }
}

struct RectConstructors;
impl LuaUserData for RectConstructors {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", new_rect);
        methods.add_function("fromCenter", from_center);
    }
}

struct FRectConstructors;
impl LuaUserData for FRectConstructors {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", new_f_rect);
    }
}

struct PointConstructors;
impl LuaUserData for PointConstructors {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", new_point);
    }
}

struct FPointConstructors;
impl LuaUserData for FPointConstructors {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", new_f_point);
    }
}

/**
    Creates the `sdl3` extension library module.

    # Errors

    Errors when out of memory.
*/
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    lua.register_userdata_type(ColorConstructors::register)
        .expect("failed to register Color userdata");
    lua.register_userdata_type(RectConstructors::register)
        .expect("failed to register Rect userdata");
    lua.register_userdata_type(FRectConstructors::register)
        .expect("failed to register FRect userdata");
    lua.register_userdata_type(PointConstructors::register)
        .expect("failed to register Point userdata");
    lua.register_userdata_type(FPointConstructors::register)
        .expect("failed to register FPoint userdata");

    TableBuilder::new(lua.clone())?
        .with_value(
            "Color",
            lua.create_any_userdata(ColorConstructors)
                .expect("failed to create Color userdata"),
        )?
        .with_value(
            "Rect",
            lua.create_any_userdata(RectConstructors)
                .expect("failed to create Rect userdata"),
        )?
        .with_value(
            "FRect",
            lua.create_any_userdata(FRectConstructors)
                .expect("failed to create FRect userdata"),
        )?
        .with_value(
            "Point",
            lua.create_any_userdata(PointConstructors)
                .expect("failed to create Point userdata"),
        )?
        .with_value(
            "FPoint",
            lua.create_any_userdata(FPointConstructors)
                .expect("failed to create FPoint userdata"),
        )?
        .with_function("init", init)?
        .build_readonly()
}

fn rgb(_: &Lua, (r, g, b): (u8, u8, u8)) -> Result<ColorWrapper, LuaError> {
    Ok(ColorWrapper(Color::RGB(r, g, b)))
}

fn rgba(_: &Lua, (r, g, b, a): (u8, u8, u8, u8)) -> Result<ColorWrapper, LuaError> {
    Ok(ColorWrapper(Color::RGBA(r, g, b, a)))
}

fn new_rect(_: &Lua, (x, y, w, h): (i32, i32, u32, u32)) -> Result<RectWrapper, LuaError> {
    Ok(RectWrapper(Rect::new(x, y, w, h)))
}

fn from_center(_: &Lua, (center, w, h): (PointWrapper, u32, u32)) -> Result<RectWrapper, LuaError> {
    Ok(RectWrapper(Rect::from_center(center.0, w, h)))
}

fn new_f_rect(_: &Lua, (x, y, w, h): (f32, f32, f32, f32)) -> Result<FRectWrapper, LuaError> {
    Ok(FRectWrapper(FRect::new(x, y, w, h)))
}

fn new_point(_: &Lua, (x, y): (i32, i32)) -> Result<PointWrapper, LuaError> {
    Ok(PointWrapper(Point::new(x, y)))
}

fn new_f_point(_: &Lua, (x, y): (f32, f32)) -> Result<FPointWrapper, LuaError> {
    Ok(FPointWrapper(FPoint::new(x, y)))
}
