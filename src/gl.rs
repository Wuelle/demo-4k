#![allow(non_snake_case)]

use core::mem;
use windows_sys::Win32::{
    Graphics::OpenGL::wglGetProcAddress,
    System::LibraryLoader::{GetProcAddress, LoadLibraryA},
};

pub struct CVoid;

pub type GlBoolean = u8;
pub type GlChar = u8;
pub type GlFloat = f32;
pub type GlEnum = u32;
pub type GlInt = i32;
pub type GlUint = u32;
pub type GlSizeI = i32;
pub type GlSizeIPtr = isize;

pub const FALSE: GlBoolean = 0;
pub const TRIANGLE_STRIP: GlEnum = 0x0005;
pub const FLOAT: GlEnum = 0x1406;
pub const COLOR: GlEnum = 0x1800;
pub const FRAGMENT_SHADER: GlEnum = 0x8B30;
pub const VERTEX_SHADER: GlEnum = 0x8B31;
pub const COMPILE_STATUS: GlEnum = 0x8B81;
pub const LINK_STATUS: GlEnum = 0x8B82;
pub const ARRAY_BUFFER: GlEnum = 0x8892;
pub const STATIC_DRAW: GlEnum = 0x88E4;

const GEN_BUFFERS_IDX: u8 = 0;
const GEN_VERTEX_ARRAYS_IDX: u8 = 1;
const BIND_VERTEX_ARRAY_IDX: u8 = 2;
const BIND_BUFFER_IDX: u8 = 3;
const BUFFER_DATA_IDX: u8 = 4;
const CREATE_PROGRAM_IDX: u8 = 5;
const ATTACH_SHADER_IDX: u8 = 6;
const LINK_PROGRAM_IDX: u8 = 7;
const DETACH_SHADER_IDX: u8 = 8;
const CREATE_SHADER_IDX: u8 = 9;
const SHADER_SOURCE_IDX: u8 = 10;
const COMPILE_SHADER_IDX: u8 = 11;
const ENABLE_VERTEX_ATTRIB_ARRAY_IDX: u8 = 12;
const VERTEX_ATTRIB_POINTER_IDX: u8 = 13;
const CLEAR_BUFFERFV_IDX: u8 = 14;
const GET_PROGRAM_IV_IDX: u8 = 15;
const GET_SHADER_IV_IDX: u8 = 16;
const GET_SHADER_INFO_LOG_IDX: u8 = 17;
const WGL_SWAP_INTERVAL_IDX: u8 = 18;
const USE_PROGRAM_IDX: u8 = 19;
const GET_UNIFORM_LOCATION_IDX: u8 = 20;
const UNIFORM_1F_IDX: u8 = 21;
const DRAW_ARRAYS_IDX: u8 = 22;
const UNIFORM_2F_IDX: u8 = 23;

const N_FUNCTIONS: usize = 24;

static mut GL_API: [usize; N_FUNCTIONS] = [0; N_FUNCTIONS];

pub unsafe fn GenBuffers(n: GlSizeI, buffers: *mut GlUint) {
    mem::transmute::<_, extern "system" fn(GlSizeI, *mut GlUint) -> ()>(
        *GL_API.get_unchecked(GEN_BUFFERS_IDX as usize),
    )(n, buffers)
}

pub unsafe fn GenVertexArrays(n: GlSizeI, arrays: *mut GlUint) {
    mem::transmute::<_, extern "system" fn(GlSizeI, *mut GlUint) -> ()>(
        *GL_API.get_unchecked(GEN_VERTEX_ARRAYS_IDX as usize),
    )(n, arrays)
}

pub unsafe fn BindVertexArray(array: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint) -> ()>(
        *GL_API.get_unchecked(BIND_VERTEX_ARRAY_IDX as usize),
    )(array)
}

pub unsafe fn BindBuffer(target: GlEnum, buffer: GlUint) {
    mem::transmute::<_, extern "system" fn(GlEnum, GlUint) -> ()>(
        *GL_API.get_unchecked(BIND_BUFFER_IDX as usize),
    )(target, buffer)
}

pub unsafe fn BufferData(target: GlEnum, size: GlSizeIPtr, data: *const CVoid, usage: GlEnum) {
    mem::transmute::<_, extern "system" fn(GlEnum, GlSizeIPtr, *const CVoid, GlEnum) -> ()>(
        *GL_API.get_unchecked(BUFFER_DATA_IDX as usize),
    )(target, size, data, usage)
}

pub unsafe fn CreateProgram() -> GlUint {
    mem::transmute::<_, extern "system" fn() -> GlUint>(
        *GL_API.get_unchecked(CREATE_PROGRAM_IDX as usize),
    )()
}

pub unsafe fn AttachShader(program: GlUint, shader: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint, GlUint) -> ()>(
        *GL_API.get_unchecked(ATTACH_SHADER_IDX as usize),
    )(program, shader)
}

pub unsafe fn LinkProgram(program: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint) -> ()>(
        *GL_API.get_unchecked(LINK_PROGRAM_IDX as usize),
    )(program)
}

pub unsafe fn DetachShader(program: GlUint, shader: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint, GlUint) -> ()>(
        *GL_API.get_unchecked(DETACH_SHADER_IDX as usize),
    )(program, shader)
}

#[must_use]
pub unsafe fn CreateShader(kind: GlEnum) -> GlUint {
    mem::transmute::<_, extern "system" fn(GlEnum) -> GlUint>(
        *GL_API.get_unchecked(CREATE_SHADER_IDX as usize),
    )(kind)
}

pub unsafe fn ShaderSource(
    shader: GlUint,
    count: GlSizeI,
    string: *const *const GlChar,
    length: *const GlInt,
) {
    mem::transmute::<_, extern "system" fn(GlUint, GlSizeI, *const *const GlChar, *const GlInt) -> ()>(
        *GL_API.get_unchecked(SHADER_SOURCE_IDX as usize),
    )(shader, count, string, length)
}

pub unsafe fn CompileShader(shader: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint) -> ()>(
        *GL_API.get_unchecked(COMPILE_SHADER_IDX as usize),
    )(shader)
}

pub unsafe fn EnableVertexAttribArray(index: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint) -> ()>(
        *GL_API.get_unchecked(ENABLE_VERTEX_ATTRIB_ARRAY_IDX as usize),
    )(index)
}

pub unsafe fn ClearBufferfv(buffer: GlEnum, drawbuffer: GlInt, value: *const GlFloat) {
    mem::transmute::<_, extern "system" fn(GlEnum, GlInt, *const GlFloat) -> ()>(
        *GL_API.get_unchecked(CLEAR_BUFFERFV_IDX as usize),
    )(buffer, drawbuffer, value)
}

pub unsafe fn VertexAttribPointer(
    index: GlUint,
    size: GlInt,
    type_: GlEnum,
    normalized: GlBoolean,
    stride: GlSizeI,
    pointer: *const CVoid,
) {
    mem::transmute::<
        _,
        extern "system" fn(GlUint, GlInt, GlEnum, GlBoolean, GlSizeI, *const CVoid) -> (),
    >(*GL_API.get_unchecked(VERTEX_ATTRIB_POINTER_IDX as usize))(
        index, size, type_, normalized, stride, pointer,
    )
}

pub unsafe fn GetProgramIv(program: GlUint, pname: GlEnum, params: *mut GlInt) {
    mem::transmute::<_, extern "system" fn(GlUint, GlEnum, *mut GlInt) -> ()>(
        *GL_API.get_unchecked(GET_PROGRAM_IV_IDX as usize),
    )(program, pname, params)
}

pub unsafe fn glGetShaderIv(shader: GlUint, sname: GlEnum, params: *mut GlInt) {
    mem::transmute::<_, extern "system" fn(GlUint, GlEnum, *mut GlInt) -> ()>(
        *GL_API.get_unchecked(GET_SHADER_IV_IDX as usize),
    )(shader, sname, params)
}

#[cfg(feature = "sanity")]
pub unsafe fn GetShaderInfoLog(
    shader: GlUint,
    bufSize: GlSizeI,
    length: *mut GlSizeI,
    infoLog: *mut GlChar,
) {
    mem::transmute::<_, extern "system" fn(GlUint, GlSizeI, *mut GlSizeI, *mut GlChar) -> ()>(
        *GL_API.get_unchecked(GET_SHADER_INFO_LOG_IDX as usize),
    )(shader, bufSize, length, infoLog)
}

pub unsafe fn wglSwapIntervalEXT(interval: GlInt) -> GlUint {
    mem::transmute::<_, extern "system" fn(GlInt) -> GlUint>(
        *GL_API.get_unchecked(WGL_SWAP_INTERVAL_IDX as usize),
    )(interval)
}

pub unsafe fn UseProgram(program: GlUint) {
    mem::transmute::<_, extern "system" fn(GlUint) -> ()>(
        *GL_API.get_unchecked(USE_PROGRAM_IDX as usize),
    )(program)
}

#[must_use]
pub unsafe fn GetUniformLocation(program: GlUint, name: *const GlChar) -> GlInt {
    mem::transmute::<_, extern "system" fn(GlUint, *const GlChar) -> GlInt>(
        *GL_API.get_unchecked(GET_UNIFORM_LOCATION_IDX as usize),
    )(program, name)
}

pub unsafe fn Uniform1f(location: GlInt, v0: GlFloat) {
    mem::transmute::<_, extern "system" fn(GlInt, GlFloat)>(
        *GL_API.get_unchecked(UNIFORM_1F_IDX as usize),
    )(location, v0)
}

pub unsafe fn DrawArrays(mode: GlEnum, first: GlInt, count: GlSizeI) {
    mem::transmute::<_, extern "system" fn(GlEnum, GlInt, GlSizeI)>(
        *GL_API.get_unchecked(DRAW_ARRAYS_IDX as usize),
    )(mode, first, count)
}

pub unsafe fn Uniform2f(location: GlInt, v0: GlFloat, v1: GlFloat) {
    mem::transmute::<_, extern "system" fn(GlInt, GlFloat, GlFloat)>(
        *GL_API.get_unchecked(UNIFORM_1F_IDX as usize),
    )(location, v0, v1)
}

pub fn init() {
    const LOAD_DESCRIPTOR: [(u8, &'static str); N_FUNCTIONS] = [
        (GEN_BUFFERS_IDX, "glGenBuffers\0"),
        (GEN_VERTEX_ARRAYS_IDX, "glGenVertexArrays\0"),
        (BIND_VERTEX_ARRAY_IDX, "glBindVertexArray\0"),
        (BIND_BUFFER_IDX, "glBindBuffer\0"),
        (BUFFER_DATA_IDX, "glBufferData\0"),
        (CREATE_PROGRAM_IDX, "glCreateProgram\0"),
        (ATTACH_SHADER_IDX, "glAttachShader\0"),
        (LINK_PROGRAM_IDX, "glLinkProgram\0"),
        (DETACH_SHADER_IDX, "glDetachShader\0"),
        (CREATE_SHADER_IDX, "glCreateShader\0"),
        (SHADER_SOURCE_IDX, "glShaderSource\0"),
        (COMPILE_SHADER_IDX, "glCompileShader\0"),
        (
            ENABLE_VERTEX_ATTRIB_ARRAY_IDX,
            "glEnableVertexAttribArray\0",
        ),
        (VERTEX_ATTRIB_POINTER_IDX, "glVertexAttribPointer\0"),
        (CLEAR_BUFFERFV_IDX, "glClearBufferfv\0"),
        (GET_PROGRAM_IV_IDX, "glGetProgramiv\0"),
        (GET_SHADER_IV_IDX, "glGetShaderiv\0"),
        (GET_SHADER_INFO_LOG_IDX, "glGetShaderInfoLog\0"),
        (WGL_SWAP_INTERVAL_IDX, "wglSwapIntervalEXT\0"),
        (USE_PROGRAM_IDX, "glUseProgram\0"),
        (GET_UNIFORM_LOCATION_IDX, "glGetUniformLocation\0"),
        (UNIFORM_1F_IDX, "glUniform1f\0"),
        (DRAW_ARRAYS_IDX, "glDrawArrays\0"),
        (UNIFORM_2F_IDX, "glUniform2f\0"),
    ];

    let handle = unsafe { LoadLibraryA("Opengl32.dll\0".as_ptr() as *const u8) };

    #[cfg(feature = "sanity")]
    if handle == 0 {
        panic!();
    }

    let mut i = 0;
    loop {
        let (index, name) = LOAD_DESCRIPTOR[i];
        unsafe {
            let addr = GetProcAddress(handle, name.as_ptr() as *const u8)
                .or_else(|| wglGetProcAddress(name.as_ptr() as *const u8))
                .unwrap() as usize;
            GL_API[index as usize] = addr;
        }

        i += 1;
        if i == LOAD_DESCRIPTOR.len() {
            break;
        }
    }
}

pub fn program_from_shaders(vtx_shader: GlUint, frag_shader: GlUint) -> GlUint {
    let program_id;
    let mut success: GlInt = 1;
    unsafe {
        program_id = CreateProgram();
        AttachShader(program_id, vtx_shader);
        AttachShader(program_id, frag_shader);
        LinkProgram(program_id);
        DetachShader(program_id, vtx_shader);
        DetachShader(program_id, frag_shader);
        GetProgramIv(program_id, LINK_STATUS, &mut success);
    }

    #[cfg(feature = "sanity")]
    if success == 0 {
        panic!();
    }

    program_id
}

pub fn shader_from_source(shader_source: &str, kind: GlEnum) -> GlUint {
    let shader_id;
    let mut success: GlInt = 1;
    unsafe {
        shader_id = CreateShader(kind);
        ShaderSource(shader_id, 1, &shader_source.as_ptr(), 0 as *const _);
        CompileShader(shader_id);
        glGetShaderIv(shader_id, COMPILE_STATUS, &mut success);
    }

    #[cfg(feature = "sanity")]
    if success == 0 {
        let mut buf = [0; 256];
        unsafe {
            GetShaderInfoLog(
                shader_id,
                buf.len() as i32,
                0 as *mut _,
                buf.as_mut_ptr() as *mut GlChar,
            );

            crate::window::show_error(buf.as_ptr());
        }
        panic!();
    }

    shader_id
}
