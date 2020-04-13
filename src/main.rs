use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HDC, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::{GetStartupInfoW, STARTUPINFOW};
use winapi::um::winbase::STARTF_USESHOWWINDOW;
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, FillRect, GetMessageW,
    PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage, COLOR_WINDOW, CW_USEDEFAULT,
    MSG, PAINTSTRUCT, SW_SHOW, WM_DESTROY, WM_PAINT, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

fn main() {
    unsafe {
        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        GetStartupInfoW(&mut startup_info);

        let ncmd_show = if startup_info.dwFlags & STARTF_USESHOWWINDOW != 0 {
            startup_info.wShowWindow as i32
        } else {
            SW_SHOW
        };

        let winapi_class_name: Vec<u16> = OsStr::new("Sample Window Class")
            .encode_wide()
            .chain(once(0))
            .collect();
        let hinstance = GetModuleHandleW(std::ptr::null());

        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(window_proc);
        wc.lpszClassName = winapi_class_name.as_ptr();
        wc.hInstance = hinstance;

        RegisterClassW(&wc);

        let window_name: Vec<u16> = OsStr::new("Rust Win32 window")
            .encode_wide()
            .chain(once(0))
            .collect();

        let hwnd = CreateWindowExW(
            0,
            winapi_class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            hinstance,
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            panic!("Something went wrong while creating a window");
        }

        ShowWindow(hwnd, ncmd_show);

        let mut msg: MSG = std::mem::zeroed();

        loop {
            let val = GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);

            if val == 0 {
                break;
            } else {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match umsg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);

            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);

            EndPaint(hwnd, &ps);
        }
        _ => { /*ignore everything else*/ }
    }

    DefWindowProcW(hwnd, umsg, wparam, lparam)
}
