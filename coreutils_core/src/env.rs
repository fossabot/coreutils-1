use std::{
    convert::From,
    env::{self, VarError},
    io::Error as IoError,
    mem::MaybeUninit,
    os::raw::c_char,
    path::PathBuf,
};

use libc::stat;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Var(VarError),
    Io(IoError),
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Error {
        Error::Var(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

pub fn current_dir_logical() -> Result<PathBuf> {
    let pwd = env::var("PWD")?;

    // Same as pwd, but null terminated
    let pwd_null = {
        let mut s = String::new();
        s.push_str(&pwd);
        s.push('\0');
        s
    };

    let (mut logical, mut physical) = (MaybeUninit::uninit(), MaybeUninit::uninit());

    // Validity check
    // if we can get both fisical and logical paths stat, check they are the same inode
    if pwd.starts_with('/') {
        let stat1 = unsafe { stat(pwd_null.as_ptr() as *const c_char, logical.as_mut_ptr()) == 0 };
        let stat2 = unsafe { stat(".\0".as_ptr() as *const c_char, physical.as_mut_ptr()) == 0 };

        let (logical, physical) = unsafe { (logical.assume_init(), physical.assume_init()) };

        if stat1 && stat2 && logical.st_ino == physical.st_ino {
            return Ok(PathBuf::from(pwd));
        }
    }
    Err(Error::Io(IoError::last_os_error()))
}
