use std::ffi::{c_int, c_void};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use futures::FutureExt;

use crate::future::{CallbackFuture, CallbackFutureResolver};
use crate::{Error, Result};

#[must_use]
#[derive(Debug)]
pub struct TaskFuture {
    state: State<()>,
}

impl TaskFuture {
    pub const fn check(&self) -> Result<()> {
        self.state.check()
    }

    pub(crate) fn create(res: Result<()>, fut: CallbackFuture<Result<()>>) -> Self {
        let state = match res {
            Ok(()) => State::Running(fut),
            Err(err) => State::Error(err),
        };
        Self { state }
    }

    pub(crate) unsafe fn resolve(userdata: *mut c_void, res: Result<()>) {
        let resolver = CallbackFutureResolver::<Result<()>>::from_raw(userdata);
        resolver.resolve(res);
    }
}

impl Future for TaskFuture {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.state.poll_unpin(cx)
    }
}

#[must_use]
#[derive(Debug)]
pub struct PacketFuture<T> {
    packet_id: u16,
    state: State<T>,
}

impl<T> PacketFuture<T> {
    pub const fn packet_id(&self) -> u16 {
        self.packet_id
    }

    pub const fn check(&self) -> Result<()> {
        self.state.check()
    }

    pub(crate) fn create(packet_id: u16, fut: CallbackFuture<Result<T>>) -> Self {
        let state = if packet_id == 0 {
            State::Error(Error::last_in_current_thread())
        } else {
            State::Running(fut)
        };
        Self { packet_id, state }
    }

    pub(crate) unsafe fn resolve(userdata: *mut c_void, res: Result<T>) {
        let resolver = CallbackFutureResolver::<Result<T>>::from_raw(userdata);
        resolver.resolve(res);
    }
}

impl PacketFuture<()> {
    pub(crate) unsafe fn resolve_with_error_code(userdata: *mut c_void, error_code: c_int) {
        let resolver = CallbackFutureResolver::<Result<()>>::from_raw(userdata);
        resolver.resolve(Error::check_rc(error_code));
    }
}

impl<T> Future for PacketFuture<T> {
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.state.poll_unpin(cx)
    }
}

#[derive(Debug)]
enum State<T> {
    Error(Error),
    Running(CallbackFuture<Result<T>>),
}

impl<T> State<T> {
    const fn check(&self) -> Result<()> {
        if let Self::Error(err) = *self {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl<T> Future for State<T> {
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this {
            Self::Error(err) => Poll::Ready(Err(*err)),
            Self::Running(fut) => fut.poll_unpin(cx),
        }
    }
}
