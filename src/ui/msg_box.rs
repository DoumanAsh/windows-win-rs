//! Message boxes APIs

use crate::sys::{HWND, MessageBoxW};
use crate::utils::Result;

use std::os::windows::ffi::OsStrExt;
use std::ffi;
use std::ptr;
use std::os::raw::{c_int, c_uint};

use crate::utils;

///Re-export WinAPI flags for `MessageBox`
pub mod flags {
    pub use crate::sys::{
        //Buttons
        MB_ABORTRETRYIGNORE,
        MB_CANCELTRYCONTINUE,
        MB_HELP,
        MB_OK,
        MB_OKCANCEL,
        MB_RETRYCANCEL,
        MB_YESNO,
        MB_YESNOCANCEL,
        //Icons
        MB_ICONEXCLAMATION,
        MB_ICONWARNING,
        MB_ICONINFORMATION,
        MB_ICONASTERISK,
        MB_ICONQUESTION,
        MB_ICONSTOP,
        MB_ICONERROR,
        MB_ICONHAND,
        //Modiality
        MB_APPLMODAL,
        MB_SYSTEMMODAL,
        MB_TASKMODAL,
    };
}

#[derive(Debug, PartialEq, Eq)]
///Result of user's interaction with message box
pub enum MsgBoxResult {
    ///Abort button is selected
    Abort,
    ///Cancel button is selected
    Cancel,
    ///Continue button is selected
    Continue,
    ///Ignore button is selected
    Ignore,
    ///No button is selected
    No,
    ///Ok button is selected
    Ok,
    ///Retry button is selected
    Retry,
    ///Try Again button is selected
    TryAgain,
    ///Yes button is selected
    Yes,
    ///Unknown result code. Non zero
    Ext(c_int),
}

impl From<c_int> for MsgBoxResult {
    fn from(value: c_int) -> MsgBoxResult {
        match value {
            1 => MsgBoxResult::Ok,
            2 => MsgBoxResult::Cancel,
            3 => MsgBoxResult::Abort,
            4 => MsgBoxResult::Retry,
            5 => MsgBoxResult::Ignore,
            6 => MsgBoxResult::Yes,
            7 => MsgBoxResult::No,
            10 => MsgBoxResult::TryAgain,
            11 => MsgBoxResult::Continue,
            value => MsgBoxResult::Ext(value),
        }
    }
}

///Message box modal dialogue
///
///If title is not specified, then Default is `Error`
///
///The default type is `flags::MB_OK`
pub struct MessageBox {
    parent: HWND,
    text: Vec<u16>,
    caption: Option<Vec<u16>>,
    flags: c_uint,
}

impl MessageBox {
    ///Creates new instance with provided text message.
    ///
    ///For multi-line text messages, just use \n
    pub fn new(text: &ffi::OsStr) -> Self {
        let mut text: Vec<u16> = text.encode_wide().collect();
        text.push(0);

        Self {
            parent: ptr::null_mut(),
            text,
            caption: None,
            flags: flags::MB_OK,
        }
    }

    #[inline]
    ///Creates informational message box with Ok button
    pub fn info<T: AsRef<ffi::OsStr>>(text: T) -> Self {
        let mut res = Self::new(text.as_ref());
        res.flags |= flags::MB_ICONINFORMATION;
        res
    }

    #[inline]
    ///Creates error message box with Ok button
    pub fn error<T: AsRef<ffi::OsStr>>(text: T) -> Self {
        let mut res = Self::new(text.as_ref());
        res.flags |= flags::MB_ICONERROR;
        res
    }

    ///Sets parent's window handle.
    pub fn parent(&mut self, parent: HWND) -> &mut Self {
        self.parent = parent;
        self
    }

    ///Sets flags value.
    pub fn set_flags(&mut self, flags: c_uint) -> &mut Self {
        self.flags = flags;
        self
    }

    ///Adds flags to existing ones.
    pub fn flags(&mut self, flags: c_uint) -> &mut Self {
        self.flags |= flags;
        self
    }

    ///Sets new text of message box
    pub fn text<T: AsRef<ffi::OsStr>>(&mut self, text: T) -> &mut Self {
        let text = text.as_ref();

        self.text.truncate(0);
        for ch in text.encode_wide() {
            self.text.push(ch);
        }
        self.text.push(0);

        self
    }

    ///Sets caption for message box.
    pub fn title<T: AsRef<ffi::OsStr>>(&mut self, text: T) -> &mut Self {
        let title = text.as_ref();

        self.caption = match self.caption.take() {
            Some(mut caption) => {
                caption.truncate(0);
                for ch in title.encode_wide() {
                    caption.push(ch);
                }
                caption.push(0);
                Some(caption)
            },
            None => {
                let mut title: Vec<u16> = title.encode_wide().collect();
                title.push(0);
                Some(title)
            },
        };

        self
    }

    ///Shows message box and returns once user closes it
    pub fn show(&self) -> Result<MsgBoxResult> {
        let caption = self.caption.as_ref().map(|caption| caption.as_ptr()).unwrap_or_else(|| ptr::null());

        match unsafe { MessageBoxW(self.parent, self.text.as_ptr(), caption, self.flags) } {
            0 => Err(utils::get_last_error()),
            n => Ok(MsgBoxResult::from(n)),
        }
    }
}
