use std::fs::File;
use std::io;
use std::io::BufRead;

use libmacchina::traits::ReadoutError;

pub type Error<T> = Result<T, ReadoutError>;

pub fn theme(keyword: &str) -> Error<String> {
    Ok(io::BufReader::new(File::open(
        dirs::config_dir()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .join("gtk-3.0")
            .join("settings.ini"),
    )?)
    .lines()
    .find(|line| match line {
        Ok(line) => line.contains(keyword),
        _ => false,
    })
    .ok_or(ReadoutError::MetricNotAvailable)??
    .split('=')
    .nth(1)
    .ok_or(ReadoutError::MetricNotAvailable)?
    .to_string())
}

use wayland_sys::client::*;
use wayland_sys::ffi_dispatch;

use nix::sys::socket::*;

use std::os::fd::AsRawFd;

pub fn window_manager() -> Error<String> {
    if !is_lib_available() {
        return Err(ReadoutError::MetricNotAvailable);
    }

    let display_ptr = unsafe {
        ffi_dispatch!(
            wayland_client_handle(),
            wl_display_connect,
            ::std::ptr::null()
        )
    };

    if display_ptr.is_null() {
        return Err(ReadoutError::MetricNotAvailable);
    }

    let display_fd =
        unsafe { ffi_dispatch!(wayland_client_handle(), wl_display_get_fd, display_ptr) }
            .as_raw_fd();

    let pid = getsockopt(display_fd, sockopt::PeerCredentials)
        .map_err(|_| ReadoutError::MetricNotAvailable)?
        .pid();

    Ok(std::fs::read_to_string(format!("/proc/{}/comm", pid))?)
}
