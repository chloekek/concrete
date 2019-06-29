//! The zmq module implements a thin, safe wrapper around libzmq.
//! The API presented by the zmq module is
//! the same as that presented by libzmq,
//! with the following exceptions:
//!
//! <ul>
//! <li>the <em>zmq_</em> prefix is stripped off of names;</li>
//! <li>strong types substitute void pointers;</li>
//! <li>slices substitute pointer–length pairs;</li>
//! <li>Drop impls automatically free resources; and</li>
//! <li>errors are reported using Result.</li>
//! </ul>

use std::ffi::CStr;
use std::ffi::c_void;
use std::io;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::os::raw::c_int;

pub struct Context { raw: *mut c_void }
pub struct Socket<'a> { raw: *mut c_void, phantom: PhantomData<&'a Context> }
impl Drop for Context { fn drop(&mut self) { unsafe { zmq_ctx_term(self.raw); } } }
impl Drop for Socket<'_> { fn drop(&mut self) { unsafe { zmq_close(self.raw); } } }
unsafe impl Send for Context { }
unsafe impl Send for Socket<'_> { }
unsafe impl Sync for Context { }

pub const REQ: c_int = 3;
pub const ROUTER: c_int = 6;
pub const PULL: c_int = 7;
pub const PUSH: c_int = 8;

pub const SNDMORE: c_int = 2;

pub fn bind(socket: &Socket, endpoint: &CStr) -> io::Result<()>
{
    let status = unsafe { zmq_bind(socket.raw, endpoint.as_ptr()) };
    if status == -1 { return Err(io::Error::last_os_error()); }
    Ok(())
}

pub fn connect(socket: &Socket, endpoint: &CStr) -> io::Result<()>
{
    let status = unsafe { zmq_connect(socket.raw, endpoint.as_ptr()) };
    if status == -1 { return Err(io::Error::last_os_error()); }
    Ok(())
}

pub fn ctx_new() -> io::Result<Context>
{
    let raw = unsafe { zmq_ctx_new() };
    if raw.is_null() { return Err(io::Error::last_os_error()); }
    Ok(Context{raw})
}

pub fn recv(socket: &Socket, buf: &mut [u8], flags: c_int) -> io::Result<c_int>
{
    let status = unsafe { zmq_recv(socket.raw, buf.as_mut_ptr() as *mut c_void, buf.len(), flags) };
    if status == -1 { return Err(io::Error::last_os_error()) }
    Ok(status)
}

pub fn send(socket: &Socket, buf: &[u8], flags: c_int) -> io::Result<()>
{
    let status = unsafe { zmq_send(socket.raw, buf.as_ptr() as *const c_void, buf.len(), flags) };
    if status == -1 { return Err(io::Error::last_os_error()); }
    Ok(())
}

pub fn socket(context: &Context, type_: c_int) -> io::Result<Socket>
{
    let raw = unsafe { zmq_socket(context.raw, type_) };
    if raw.is_null() { return Err(io::Error::last_os_error()); }
    Ok(Socket{raw, phantom: PhantomData})
}

extern "C"
{
    fn zmq_bind(socket: *mut c_void, endpoint: *const c_char) -> c_int;
    fn zmq_close(socket: *mut c_void) -> c_int;
    fn zmq_connect(socket: *mut c_void, endpoint: *const c_char) -> c_int;
    fn zmq_ctx_new() -> *mut c_void;
    fn zmq_ctx_term(context: *mut c_void) -> c_int;
    fn zmq_recv(socket: *mut c_void, buf: *mut c_void, len: usize, flags: c_int) -> c_int;
    fn zmq_send(socket: *mut c_void, buf: *const c_void, len: usize, flags: c_int) -> c_int;
    fn zmq_socket(context: *mut c_void, type_: c_int) -> *mut c_void;
}
