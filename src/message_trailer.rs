use libc;

use crate::raw;

use crate::util::Binding;

/// A message trailer key->value pair.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MessageTrailer {
    raw: raw::git_message_trailer
}

impl Binding for MessageTrailer {
    type Raw = *const raw::git_message_trailer;

    unsafe fn from_raw(trailer: *const raw::git_message_trailer) -> MessageTrailer {
        MessageTrailer { raw: *trailer }
    }
    fn raw(&self) -> *const raw::git_message_trailer {
        &self.raw as *const _
    }
}