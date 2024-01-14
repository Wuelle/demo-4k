use crate::{gl, sanity_assert};
use windows_sys::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::{Gdi::*, OpenGL::*},
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};

use core::{mem, ptr};

#[cfg(feature = "sanity")]
pub unsafe fn show_error(message: *const u8) {
    MessageBoxA(
        0 as HWND,
        message,
        "Window::create\0".as_ptr() as *const u8,
        MB_ICONERROR,
    );
}

pub fn handle_message(_window: HWND) -> bool {
    let mut msg: mem::MaybeUninit<MSG> = mem::MaybeUninit::uninit();
    loop {
        unsafe {
            if PeekMessageA(msg.as_mut_ptr(), 0 as HWND, 0, 0, PM_REMOVE) == 0 {
                return true;
            }
            let msg = msg.assume_init();
            if msg.message == WM_QUIT {
                return false;
            }

            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

#[must_use]
pub fn size(window: HWND) -> (i32, i32) {
    let mut rect: mem::MaybeUninit<RECT> = mem::MaybeUninit::uninit();
    let _status = unsafe { GetWindowRect(window, rect.as_mut_ptr()) };
    sanity_assert!(_status != 0);

    let rect = unsafe { rect.assume_init() };
    (rect.right - rect.left, rect.top - rect.bottom)
}

#[must_use]
pub fn create() -> (HWND, HDC) {
    unsafe {
        let instance = GetModuleHandleA(ptr::null());

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(0, IDC_ARROW),
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0,
            hbrBackground: 0,
            lpszMenuName: ptr::null(),
        };

        let _atom = RegisterClassA(&wc);
        sanity_assert!(_atom != 0);

        #[cfg(feature = "sanity")]
        let title = s!("Demo intro (RUNNING IN SANITY MODE, DO NOT SUBMIT)");
        #[cfg(not(feature = "sanity"))]
        let title = s!("");

        let h_wnd = CreateWindowExA(
            0,
            window_class,
            title,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            0 as HWND,
            0 as HMENU,
            instance,
            ptr::null(),
        );

        let h_dc: HDC = GetDC(h_wnd);

        let mut pfd: PIXELFORMATDESCRIPTOR = mem::zeroed();
        pfd.nSize = mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 32;
        pfd.cAlphaBits = 8;
        pfd.cDepthBits = 24;

        let pfd_id = ChoosePixelFormat(h_dc, &pfd);
        sanity_assert!(pfd_id != 0);

        let _status = SetPixelFormat(h_dc, pfd_id, &pfd);
        sanity_assert!(_status != 0);

        let gl_context: HGLRC = wglCreateContext(h_dc);
        sanity_assert!(gl_context != 0);

        let _status = wglMakeCurrent(h_dc, gl_context);
        sanity_assert!(_status != 0);

        gl::init();
        gl::wglSwapIntervalEXT(1);

        (h_wnd, h_dc)
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                ValidateRect(window, ptr::null());
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
