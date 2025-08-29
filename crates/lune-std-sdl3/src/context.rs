use std::{cell::RefCell, os::raw, rc::Rc};

use image;
use lune_utils::TableBuilder;
use mlua::prelude::*;
use once_cell::sync::Lazy;
use sdl3::{event::*, pixels::*, rect::*, render::*, video::*, *};

pub struct SdlWrapper(pub Sdl);
pub struct EventPumpWrapper(pub Rc<RefCell<EventPump>>);
pub struct EventWrapper(pub Event);
pub struct VideoWrapper(pub VideoSubsystem);
pub struct WindowWrapper(pub Window);
pub struct CanvasWrapper(pub Rc<RefCell<WindowCanvas>>);
pub struct ColorWrapper(pub Color);
pub struct RectWrapper(pub Rect);
pub struct FRectWrapper(pub FRect);
pub struct PointWrapper(pub Point);
pub struct FPointWrapper(pub FPoint);

pub fn init(_: &Lua, (): ()) -> Result<SdlWrapper, LuaError> {
    match sdl3::init() {
        Ok(sdl) => Ok(SdlWrapper(sdl)),
        Err(e) => Err(LuaError::RuntimeError(
            "SDL failed to initialize: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

impl FromLua for SdlWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Sdl".to_string(),
            message: Some("failed to create Sdl userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<SdlWrapper>() {
                    return Ok(SdlWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for SdlWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_function("video", video);
            registry.add_function("eventPump", event_pump);
        })
        .expect("failed to register userdata for Sdl");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for EventPumpWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "EventPump".to_string(),
            message: Some("failed to create EventPump userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<EventPumpWrapper>() {
                    return Ok(EventPumpWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for EventPumpWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_function("poll", poll);
        })
        .expect("failed to register userdata for EventPump");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for VideoWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "VideoSubsystem".to_string(),
            message: Some("failed to create VideoSubsystem userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<VideoWrapper>() {
                    return Ok(VideoWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for VideoWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_function("window", window);
        })
        .expect("failed to register userdata for VideoSubsystem");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for WindowWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Window".to_string(),
            message: Some("failed to create Window userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<WindowWrapper>() {
                    return Ok(WindowWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for WindowWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_function("canvas", canvas);
        })
        .expect("failed to register userdata for Window");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for CanvasWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Canvas".to_string(),
            message: Some("failed to create Canvas userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<CanvasWrapper>() {
                    return Ok(CanvasWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for CanvasWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_function("clear", canvas_clear);
            registry.add_function("present", canvas_present);
            registry.add_function("setDrawColor", canvas_set_draw_color);
            registry.add_function("drawRect", canvas_draw_rect);
            registry.add_function("fillRect", canvas_fill_rect);
            registry.add_function("drawLine", canvas_draw_line);
            // registry.add_function("createTexture", canvas_create_texture);
            // registry.add_function("drawTexture", canvas_draw_texture);
        })
        .expect("failed to register userdata for Canvas");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for ColorWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Color".to_string(),
            message: Some("failed to create Color userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<ColorWrapper>() {
                    return Ok(ColorWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for ColorWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_field("r", self.0.r);
            registry.add_field("g", self.0.g);
            registry.add_field("b", self.0.b);
            registry.add_field("a", self.0.a);
        })
        .expect("failed to register userdata for Color");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for RectWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Rect".to_string(),
            message: Some("failed to create Rect userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<RectWrapper>() {
                    return Ok(RectWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for RectWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_field("x", self.0.x);
            registry.add_field("y", self.0.y);
            registry.add_field("w", self.0.w);
            registry.add_field("h", self.0.h);
        })
        .expect("failed to register userdata for Rect");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for FRectWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "FRect".to_string(),
            message: Some("failed to create FRect userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<FRectWrapper>() {
                    return Ok(FRectWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for FRectWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_field("x", self.0.x);
            registry.add_field("y", self.0.y);
            registry.add_field("w", self.0.w);
            registry.add_field("h", self.0.h);
        })
        .expect("failed to register userdata for FRect");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for PointWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Point".to_string(),
            message: Some("failed to create Point userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<PointWrapper>() {
                    return Ok(PointWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for PointWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_field("x", self.0.x);
            registry.add_field("y", self.0.y);
        })
        .expect("failed to register userdata for Point");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

impl FromLua for FPointWrapper {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let conversion_error = LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "FPoint".to_string(),
            message: Some("failed to create FPoint userdata".to_string()),
        };

        match value {
            LuaValue::UserData(ref ud) => {
                if let Ok(ud_ref) = ud.borrow::<FPointWrapper>() {
                    return Ok(FPointWrapper(ud_ref.0.clone()));
                }
                Err(conversion_error)
            }
            _ => Err(conversion_error),
        }
    }
}

impl IntoLua for FPointWrapper {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        lua.register_userdata_type(|registry: &mut LuaUserDataRegistry<Self>| {
            registry.add_field("x", self.0.x);
            registry.add_field("y", self.0.y);
        })
        .expect("failed to register userdata for FPoint");

        match lua.create_any_userdata(self) {
            Err(e) => Err(e),
            Ok(ud) => Ok(LuaValue::UserData(ud)),
        }
    }
}

macro_rules! event_type_from_event {
    ($event:expr, [ $( $variant:ident ),* $(,)? ]) => {
        match $event {
            $(
                Event::$variant { .. } => EventType::$variant,
            )*
            _ => EventType::User,
        }
    };
}

fn raw_event_id(event: Rc<RefCell<Event>>) -> u32 {
    let event_type = event_type_from_event!(
        *event.borrow(),
        [
            Quit,
            AppTerminating,
            AppLowMemory,
            AppWillEnterBackground,
            AppDidEnterBackground,
            AppWillEnterForeground,
            AppDidEnterForeground,
            // Window,
            KeyDown,
            KeyUp,
            TextEditing,
            TextInput,
            MouseMotion,
            MouseButtonDown,
            MouseButtonUp,
            MouseWheel,
            JoyAxisMotion,
            JoyHatMotion,
            JoyButtonDown,
            JoyButtonUp,
            JoyDeviceAdded,
            JoyDeviceRemoved,
            ControllerAxisMotion,
            ControllerButtonDown,
            ControllerButtonUp,
            ControllerDeviceAdded,
            ControllerDeviceRemoved,
            ControllerDeviceRemapped,
            ControllerTouchpadMotion,
            ControllerTouchpadDown,
            ControllerButtonUp,
            FingerMotion,
            FingerDown,
            FingerUp,
            // DollarRecord,
            // MultiGesture,
            ClipboardUpdate,
            DropFile,
            DropText,
            DropBegin,
            DropComplete,
            AudioDeviceAdded,
            AudioDeviceRemoved,
            RenderDeviceReset,
            RenderDeviceReset
        ]
    );
    u32::from(event_type).into()
}

fn event_info(lua: Lua, event_ref: Rc<RefCell<Event>>) -> Result<TableBuilder, LuaError> {
    TableBuilder::new(lua)?
        .with_value("id", raw_event_id(event_ref.clone()))?
        .with_value("timestamp", event_ref.borrow().get_timestamp())
        .into_lua_err()
}

fn window_event_info(
    lua: Lua,
    event: Rc<RefCell<Event>>,
    window_id: u32,
) -> Result<TableBuilder, LuaError> {
    event_info(lua, event)?.with_value("windowID", window_id)
}

fn keypress_event_info(
    lua: &Lua,
    event: Rc<RefCell<Event>>,
    window_id: u32,
    keycode: Option<keyboard::Keycode>,
    scancode: Option<keyboard::Scancode>,
    keymod: keyboard::Mod,
    repeat: bool,
    which: u32,
) -> Result<LuaTable, LuaError> {
    let mut builder = window_event_info(lua.clone(), event, window_id)?
        .with_value("keymod", keymod.bits())?
        .with_value("repeated", repeat)?
        .with_value("which", which);

    if let Some(code) = keycode {
        builder = builder?.with_value("keycode", code.to_ll())
    }
    if let Some(code) = scancode {
        builder = builder?.with_value("scancode", code.to_i32())
    }

    builder?.build_readonly()
}

impl IntoLua for EventWrapper {
    fn into_lua(self, lua: &Lua) -> Result<LuaValue, LuaError> {
        let event = Rc::new(RefCell::new(self.0.clone()));
        let result = match self.0 {
            Event::Window {
                timestamp,
                window_id,
                win_event,
            } => window_event_info(lua.clone(), event, window_id)?.build_readonly(),

            Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
                which,
                raw,
            } => keypress_event_info(
                lua, event, window_id, keycode, scancode, keymod, repeat, which,
            ),
            Event::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
                which,
                raw,
            } => keypress_event_info(
                lua, event, window_id, keycode, scancode, keymod, repeat, which,
            ),
            Event::TextEditing {
                timestamp,
                window_id,
                text,
                start,
                length,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::TextInput {
                timestamp,
                window_id,
                text,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::MouseMotion {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
                mouse_x,
                mouse_y,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::JoyAxisMotion {
                timestamp,
                which,
                axis_idx,
                value,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::JoyHatMotion {
                timestamp,
                which,
                hat_idx,
                state,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::JoyButtonDown {
                timestamp,
                which,
                button_idx,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::JoyButtonUp {
                timestamp,
                which,
                button_idx,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::JoyDeviceAdded { timestamp, which } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::JoyDeviceRemoved { timestamp, which } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::ControllerAxisMotion {
                timestamp,
                which,
                axis,
                value,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::ControllerButtonDown {
                timestamp,
                which,
                button,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::ControllerButtonUp {
                timestamp,
                which,
                button,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::ControllerDeviceAdded { timestamp, which } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::ControllerDeviceRemoved { timestamp, which } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::ControllerDeviceRemapped { timestamp, which } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::ControllerTouchpadDown {
                timestamp,
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::ControllerTouchpadMotion {
                timestamp,
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::ControllerTouchpadUp {
                timestamp,
                which,
                touchpad,
                finger,
                x,
                y,
                pressure,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::FingerDown {
                timestamp,
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::FingerUp {
                timestamp,
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::FingerMotion {
                timestamp,
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::DollarRecord {
                timestamp,
                touch_id,
                gesture_id,
                num_fingers,
                error,
                x,
                y,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::MultiGesture {
                timestamp,
                touch_id,
                d_theta,
                d_dist,
                x,
                y,
                num_fingers,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::ClipboardUpdate { timestamp } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::DropFile {
                timestamp,
                window_id,
                filename,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::DropText {
                timestamp,
                window_id,
                filename,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::DropBegin {
                timestamp,
                window_id,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::DropComplete {
                timestamp,
                window_id,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::AudioDeviceAdded {
                timestamp,
                which,
                iscapture,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::AudioDeviceRemoved {
                timestamp,
                which,
                iscapture,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenProximityIn {
                timestamp,
                which,
                window,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenProximityOut {
                timestamp,
                which,
                window,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenDown {
                timestamp,
                which,
                window,
                x,
                y,
                eraser,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenUp {
                timestamp,
                which,
                window,
                x,
                y,
                eraser,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenMotion {
                timestamp,
                which,
                window,
                x,
                y,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenButtonUp {
                timestamp,
                which,
                window,
                x,
                y,
                button,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenButtonDown {
                timestamp,
                which,
                window,
                x,
                y,
                button,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::PenAxis {
                timestamp,
                which,
                window,
                x,
                y,
                axis,
                value,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::RenderTargetsReset { timestamp } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::RenderDeviceReset { timestamp } => {
                event_info(lua.clone(), event)?.build_readonly()
            }
            Event::User {
                timestamp,
                window_id,
                type_,
                code,
                data1,
                data2,
            } => event_info(lua.clone(), event)?.build_readonly(),
            Event::Unknown { timestamp, type_ } => event_info(lua.clone(), event)?.build_readonly(),
            Event::Display {
                timestamp,
                display,
                display_event,
            } => event_info(lua.clone(), event)?.build_readonly(),

            _ => event_info(lua.clone(), event)?.build_readonly(),
        };

        result.map(|table| LuaValue::Table(table))
    }
}

fn event_pump(_: &Lua, sdl: SdlWrapper) -> Result<EventPumpWrapper, LuaError> {
    match sdl.0.event_pump() {
        Ok(ep) => Ok(EventPumpWrapper(Rc::new(RefCell::new(ep)))),
        Err(e) => Err(LuaError::RuntimeError(
            "Failed to initialize EventPump: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

fn poll(_: &Lua, event_pump: EventPumpWrapper) -> Result<Vec<EventWrapper>, LuaError> {
    let events = event_pump
        .0
        .borrow_mut()
        .poll_iter()
        .map(EventWrapper)
        .collect::<Vec<EventWrapper>>();

    Ok(events)
}

fn video(_: &Lua, sdl: SdlWrapper) -> Result<VideoWrapper, LuaError> {
    match sdl.0.video() {
        Ok(v) => Ok(VideoWrapper(v)),
        Err(e) => Err(LuaError::RuntimeError(
            "Failed to initialize VideoSubsystem: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

fn window(
    _: &Lua,
    (video, title, width, height): (VideoWrapper, String, u32, u32),
) -> Result<WindowWrapper, LuaError> {
    match video
        .0
        .window(&title, width, height)
        .position_centered()
        .build()
    {
        Ok(w) => Ok(WindowWrapper(w)),
        Err(e) => Err(LuaError::RuntimeError(
            "Failed to create Window: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

fn canvas(_: &Lua, window: WindowWrapper) -> Result<CanvasWrapper, LuaError> {
    let canvas = window.0.into_canvas();
    Ok(CanvasWrapper(Rc::new(RefCell::new(canvas))))
}

fn canvas_present(_: &Lua, canvas_wrapper: CanvasWrapper) -> Result<bool, LuaError> {
    Ok(canvas_wrapper.0.borrow_mut().present())
}

fn canvas_clear(_: &Lua, canvas_wrapper: CanvasWrapper) -> Result<(), LuaError> {
    Ok(canvas_wrapper.0.borrow_mut().clear())
}

fn canvas_set_draw_color(
    _: &Lua,
    (canvas_wrapper, color): (CanvasWrapper, ColorWrapper),
) -> Result<(), LuaError> {
    Ok(canvas_wrapper.0.borrow_mut().set_draw_color(color.0))
}

fn canvas_draw_line(
    _: &Lua,
    (canvas_wrapper, p1, p2): (CanvasWrapper, FPointWrapper, FPointWrapper),
) -> Result<(), LuaError> {
    match canvas_wrapper.0.borrow_mut().draw_line(p1.0, p2.0) {
        Ok(_) => Ok(()),
        Err(e) => Err(LuaError::RuntimeError(
            "SDL draw line error: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

fn canvas_draw_rect(
    _: &Lua,
    (canvas_wrapper, rect): (CanvasWrapper, FRectWrapper),
) -> Result<(), LuaError> {
    match canvas_wrapper.0.borrow_mut().draw_rect(rect.0) {
        Ok(_) => Ok(()),
        Err(e) => Err(LuaError::RuntimeError(
            "SDL draw rect error: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

fn canvas_fill_rect(
    _: &Lua,
    (canvas_wrapper, rect): (CanvasWrapper, FRectWrapper),
) -> Result<(), LuaError> {
    match canvas_wrapper.0.borrow_mut().fill_rect(rect.0) {
        Ok(_) => Ok(()),
        Err(e) => Err(LuaError::RuntimeError(
            "SDL fill rect error: ".to_string() + e.to_string().as_ref(),
        )),
    }
}

// static IMAGE_CACHE: Lazy<Mutex<HashMap<String, image::RgbaImage>>> =
//     Lazy::new(|| Mutex::new(HashMap::new()));

// fn canvas_create_texture(
//     _: &Lua,
//     (canvas_wrapper, path, rect): (CanvasWrapper, String, Option<FRectWrapper>),
// ) -> Result<TextureWrapper, LuaError> {
//     let img = {
//         let mut cache = IMAGE_CACHE.lock().unwrap();
//         if let Some(cached) = cache.get(&path) {
//             cached.clone()
//         } else {
//             let img = image::open(Path::new(&path)).into_lua_err()?.to_rgba8();
//             cache.insert(path.clone(), img.clone());
//             img
//         }
//     };

//     let (width, height) = img.dimensions();
//     let mut pixels = img.into_raw();
//     let surface = Surface::from_data_pixelmasks(
//         &mut pixels,
//         width,
//         height,
//         width * 4,
//         &PixelMasks {
//             rmask: 0x000000ff,
//             gmask: 0x0000ff00,
//             bmask: 0x00ff0000,
//             amask: 0xff000000,
//             bpp: 32,
//         },
//     )
//     .into_lua_err()?;

//     let texture = {
//         let texture_creator = canvas_wrapper.texture_creator.borrow();
//         texture_creator
//             .create_texture_from_surface(&surface)
//             .into_lua_err()?
//     };

//     Ok(TextureWrapper(Rc::new(RefCell::new(texture))))
// }

// fn canvas_draw_texture(
//     _: &Lua,
//     (canvas_wrapper, texture, rect): (CanvasWrapper, TextureWrapper, Option<FRectWrapper>),
// ) -> Result<(), LuaError> {
//     let destination = rect.map(|r| r.0);
//     canvas_wrapper
//         .canvas
//         .borrow_mut()
//         .copy(&texture.0.borrow_mut(), None, destination)
//         .into_lua_err()
// }
