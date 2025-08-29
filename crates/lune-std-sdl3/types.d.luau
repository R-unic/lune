--!nocheck
local sdl3 = {}

local EventPump = {}
EventPump.__index = EventPump

export type EventPump = setmetatable<{}, typeof(EventPump)>

export type Event = {
	read id: number,
	read timestamp: number,
}

export type WindowEvent = Event & {
	read windowID: number,
}

export type KeypressEvent = WindowEvent & {
	read keycode: number,
	read scancode: number,
	read keymod: number,
	read repeated: boolean,
	read which: number,
}

function EventPump:poll(): { Event }
	return nil :: any
end

sdl3.Color = {}

function sdl3.Color.rgb(r: number, g: number, b: number): Color
	return nil :: any
end

function sdl3.Color.rgba(r: number, g: number, b: number, a: number): Color
	return nil :: any
end

export type Color = {
	read r: number,
	read g: number,
	read b: number,
	read a: number,
}

sdl3.Rect = {}

function sdl3.Rect.new(x: number, y: number, w: number, h: number): Rect
	return nil :: any
end

function sdl3.Rect.fromCenter(center: Point, w: number, h: number): Rect
	return nil :: any
end

type Rect = {
	read x: number,
	read y: number,
	read w: number,
	read h: number,
}

sdl3.FRect = {}

function sdl3.FRect.new(x: number, y: number, w: number, h: number): FRect
	return nil :: any
end

type FRect = Rect

sdl3.Point = {}

function sdl3.Point.new(x: number, y: number): Point
	return nil :: any
end

type Point = {
	read x: number,
	read y: number,
}

sdl3.FPoint = {}

function sdl3.FPoint.new(x: number, y: number): FPoint
	return nil :: any
end

type FPoint = Point

local Texture = {}
Texture.__index = Texture

export type Texture = setmetatable<{}, typeof(Texture)>

local Canvas = {}
Canvas.__index = Canvas

export type Canvas = setmetatable<{}, typeof(Canvas)>

function Canvas:setDrawColor(color: Color): () end
function Canvas:drawRect(rect: FRect): () end
function Canvas:fillRect(rect: FRect): () end
function Canvas:drawLine(p1: Point, p2: Point): () end
function Canvas:createTexture(path: string): Texture end
function Canvas:drawTexture(texture: Texture, rect: FRect?): () end

function Canvas:clear(): () end

function Canvas:present(): () end

local Window = {}
Window.__index = Window

export type Window = setmetatable<{}, typeof(Window)>

function Window:canvas(): Canvas
	return nil :: any
end

local VideoSubsystem = {}
VideoSubsystem.__index = VideoSubsystem

export type VideoSubsystem = setmetatable<{}, typeof(VideoSubsystem)>

function VideoSubsystem:window(title: string, width: number, height: number): Window
	return nil :: any
end

local Sdl = {}
Sdl.__index = Sdl

export type Sdl = setmetatable<{}, typeof(Sdl)>

function Sdl:video(): VideoSubsystem
	return nil :: any
end

function Sdl:eventPump(): EventPump
	return nil :: any
end

function sdl3.init(): Sdl
	return nil :: any
end

export type sdl3 = typeof(sdl3)

return sdl3
