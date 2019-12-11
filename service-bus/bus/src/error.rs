#![allow(unused)]

use actix::MailboxError;
use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "bus connection fail: {}", _0)]
    BusConnectionFail(io::Error),
    #[fail(display = "Mailbox has closed")]
    Closed,
    #[fail(display = "has closed")]
    NoEndpoint,
    #[fail(display = "bad content {}", _0)]
    BadContent(#[cause] rmp_serde::decode::Error),
    #[fail(display = "{}", _0)]
    EncodingProblem(String),
    #[fail(display = "Message delivery timed out")]
    Timeout,
    #[fail(display = "bad request: {}", _0)]
    GsbBadRequest(String),
    #[fail(display = "already registered: {}", _0)]
    GsbAlreadyRegistered(String),
    #[fail(display = "{}", _0)]
    GsbFailure(String),
}

impl From<MailboxError> for Error {
    fn from(e: MailboxError) -> Self {
        match e {
            MailboxError::Closed => Error::Closed,
            MailboxError::Timeout => Error::Timeout,
        }
    }
}

impl From<futures_01::sync::oneshot::Canceled> for Error {
    fn from(_: futures_01::sync::oneshot::Canceled) -> Self {
        Error::Closed
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Error::BadContent(e)
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Error::EncodingProblem(format!("{}", e))
    }
}
