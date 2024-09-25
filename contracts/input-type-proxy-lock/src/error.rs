use ckb_std::error::SysError;
use ckb_std::error::SysError::{Encoding, IndexOutOfBound, InvalidFd, ItemMissing, MaxFdsCreated, MaxVmsSpawned, OtherEndClosed, WaitFailure};

/// Error
#[repr(i8)]
#[derive(Debug)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    Overflow,

    InvalidUnlock,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            WaitFailure => Self::Encoding,
            InvalidFd =>Self::Encoding,
            OtherEndClosed => Self::Encoding,
            MaxVmsSpawned => Self::Encoding,
            MaxFdsCreated => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
            _ => todo!(),
            // SysError::TypeIDError => {}
        }
    }
}