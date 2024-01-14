#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(
    lang_items,
    alloc_error_handler,
    core_intrinsics,
    cfg_match,
    const_mut_refs,
    const_fn_floating_point_arithmetic
)]
#![windows_subsystem = "windows"]

mod gl;
mod music;
mod random;
mod window;

use core::{
    alloc::{self, GlobalAlloc},
    hint::unreachable_unchecked,
    mem,
    panic::PanicInfo,
};

use windows_sys::Win32::{
    Graphics::OpenGL::SwapBuffers,
    System::{
        Memory::{GetProcessHeap, HeapAlloc},
        Threading::ExitProcess,
    },
};

macro_rules! sanity_assert {
    ($cond: expr) => {
        #[cfg(feature = "sanity")]
        if !$cond {
            panic!();
        }
    };
}
use sanity_assert;

struct CustomAllocator;

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        HeapAlloc(GetProcessHeap(), 0, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::Layout) {}
}

#[global_allocator]
static ALLOC: CustomAllocator = CustomAllocator;

#[alloc_error_handler]
fn alloc_error(_: alloc::Layout) -> ! {
    unsafe {
        unreachable_unchecked();
    }
}

cfg_match! {
    cfg(feature = "sanity") => {
        #[panic_handler]
        fn panic(_info: &PanicInfo) -> ! {
            unsafe {
                window::show_error("panic\0".as_ptr());
                ExitProcess(1);
            };
        }
    }
    _ => {
        #[panic_handler]
        fn panic(_info: &PanicInfo) -> ! {
            // "Don't panic"
            unsafe {
                unreachable_unchecked();
            };
        }
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        if *((a as usize + 1) as *const u8) != *((b as usize + 1) as *const u8) {
            return 0;
        }
        i += 1;
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = c as u8;
        i += 1;
    }
    dest
}

const TICKRATE: f32 = 1. / 60.;
const DEMO_LENGTH: usize = 75;
pub(crate) const MUSIC_LENGTH: usize = DEMO_LENGTH * 3; // Account for lag

#[no_mangle]
pub unsafe extern "C" fn WinMainCRTStartup() -> i32 {
    let (h_wnd, h_dc) = window::create();

    music::play();

    let vtx_shader_src: &'static str =
        concat!(include_str!("../shaders/vertex_minified.glsl"), "\0");
    let frag_shader_src: &'static str = concat!(include_str!("../shaders/fragment_minified.glsl"), "\0\0");

    let vtx_coords: [[gl::GlFloat; 3]; 4] =
        [[-1., -1., 0.], [1., -1., 0.], [-1., 1., 0.], [1., 1., 0.]];

    let vtx_shader = gl::shader_from_source(vtx_shader_src, gl::VERTEX_SHADER);
    let frag_shader = gl::shader_from_source(frag_shader_src, gl::FRAGMENT_SHADER);
    let shader_prog = gl::program_from_shaders(vtx_shader, frag_shader);

    // OpenGL setup
    let mut vertex_buffer_id: gl::GlUint = 0;
    let mut vertex_array_id: gl::GlUint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer_id);
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of::<gl::GlFloat>() as isize * 12,
            vtx_coords.as_ptr() as *const gl::CVoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<gl::GlFloat>() as gl::GlInt,
            0 as *const gl::CVoid,
        );
    }

    let mut tick = 0.0;
    loop {
        if !window::handle_message(h_wnd) {
            break;
        }

        let rgba = &[0., 0., 0., 0.];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, rgba as *const _);
            gl::UseProgram(shader_prog);

            let tick_loc = gl::GetUniformLocation(shader_prog, "v\0".as_ptr());
            gl::Uniform1f(tick_loc, tick);

            gl::BindVertexArray(vertex_array_id);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            SwapBuffers(h_dc);
        }

        tick += TICKRATE;

        if tick > DEMO_LENGTH as f32 {
            break;
        }
    }

    ExitProcess(0);
}
